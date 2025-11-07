use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{io, time::Duration};

/// Represents a single TUI session with its own pseudo-terminal
struct TuiSession {
    name: String,
    command: String,
    output_buffer: Arc<Mutex<Vec<u8>>>,
    pty_writer: Arc<Mutex<Box<dyn Write + Send>>>,
    _pty_pair: Option<Box<dyn portable_pty::Child + Send>>,
}

impl TuiSession {
    fn new(name: String, command: String, args: Vec<String>) -> Result<Self> {
        let pty_system = NativePtySystem::default();
        
        // Create a new pseudo-terminal with a reasonable size
        let pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let output_buffer = Arc::new(Mutex::new(Vec::new()));
        let output_buffer_clone = Arc::clone(&output_buffer);

        // Take the writer for input forwarding
        let pty_writer = Arc::new(Mutex::new(pair.master.take_writer()?));

        // Spawn the reader thread for PTY output using blocking thread pool
        let mut reader = pair.master.try_clone_reader()?;
        tokio::task::spawn_blocking(move || {
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        if let Ok(mut buffer) = output_buffer_clone.lock() {
                            buffer.extend_from_slice(&buf[..n]);
                            // Keep buffer size reasonable (last 100KB)
                            let buffer_len = buffer.len();
                            if buffer_len > 100_000 {
                                buffer.drain(..buffer_len - 100_000);
                            }
                        }
                    }
                    Ok(_) => break,
                    Err(_) => break,
                }
                // Small sleep to avoid busy-waiting
                std::thread::sleep(Duration::from_millis(10));
            }
        });

        // Spawn the command
        let mut cmd = CommandBuilder::new(&command);
        for arg in args {
            cmd.arg(arg);
        }
        
        let child = pair.slave.spawn_command(cmd)?;

        Ok(Self {
            name,
            command,
            output_buffer,
            pty_writer,
            _pty_pair: Some(child),
        })
    }

    fn get_output(&self) -> String {
        if let Ok(buffer) = self.output_buffer.lock() {
            String::from_utf8_lossy(&buffer).to_string()
        } else {
            String::new()
        }
    }

    fn write_input(&self, data: &[u8]) -> Result<()> {
        if let Ok(mut writer) = self.pty_writer.lock() {
            writer.write_all(data)?;
            writer.flush()?;
        }
        Ok(())
    }
}

/// Main application state
struct App {
    sessions: Vec<TuiSession>,
    selected_tab: usize,
    should_quit: bool,
}

impl App {
    async fn new() -> Result<Self> {
        // Example sessions - you can modify these or make them configurable
        // For demo purposes, we'll use simple commands that produce output
        
        let sessions = vec![
            // Session 1: watch date command (simulates a live updating TUI)
            TuiSession::new(
                "Watch Date".to_string(),
                "watch".to_string(),
                vec!["-n".to_string(), "1".to_string(), "date".to_string()],
            )?,
            // Session 2: top command (actual TUI)
            TuiSession::new(
                "System Monitor".to_string(),
                "top".to_string(),
                vec!["-b".to_string(), "-d".to_string(), "1".to_string()],
            )?,
            // Session 3: tail of a log file (or create one)
            TuiSession::new(
                "Log Viewer".to_string(),
                "bash".to_string(),
                vec![
                    "-c".to_string(),
                    "while true; do echo \"[$(date)] Log entry $RANDOM\"; sleep 1; done".to_string(),
                ],
            )?,
            // Session 4: Shell session
            TuiSession::new(
                "Shell".to_string(),
                "bash".to_string(),
                vec!["--norc".to_string(), "-i".to_string()],
            )?,
        ];

        Ok(Self {
            sessions,
            selected_tab: 0,
            should_quit: false,
        })
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        // Check for navigation keys first (these take priority)
        match (key.code, key.modifiers) {
            // Application control keys (don't forward to session)
            (KeyCode::Char('q'), KeyModifiers::CONTROL) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return;
            }
            // Navigation: Ctrl+Arrow keys or Ctrl+1-9 only
            (KeyCode::Right, KeyModifiers::CONTROL) => {
                self.selected_tab = (self.selected_tab + 1) % self.sessions.len();
                return;
            }
            (KeyCode::Left, KeyModifiers::CONTROL) => {
                self.selected_tab = if self.selected_tab == 0 {
                    self.sessions.len() - 1
                } else {
                    self.selected_tab - 1
                };
                return;
            }
            (KeyCode::Char(c), KeyModifiers::CONTROL) if ('1'..='9').contains(&c) => {
                let idx = (c as usize) - ('1' as usize);
                if idx < self.sessions.len() {
                    self.selected_tab = idx;
                }
                return;
            }
            _ => {}
        }
        
        // Forward all other keys (including Tab, Shift+Tab, etc.) to the selected session
        if let Some(session) = self.sessions.get(self.selected_tab) {
            if let Some(bytes) = key_event_to_bytes(&key) {
                let _ = session.write_input(&bytes);
            }
        }
    }
}

/// Convert a KeyEvent to bytes that should be sent to the PTY
fn key_event_to_bytes(key: &event::KeyEvent) -> Option<Vec<u8>> {
    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Handle control characters
                match c {
                    // Ctrl+A through Ctrl+Z
                    'a'..='z' | 'A'..='Z' => {
                        let byte = (c.to_ascii_lowercase() as u8) - b'a' + 1;
                        Some(vec![byte])
                    }
                    // Special control characters
                    '@' => Some(vec![0x00]),     // Ctrl+@: NUL
                    '[' => Some(vec![0x1b]),     // Ctrl+[: ESC
                    '\\' => Some(vec![0x1c]),    // Ctrl+\: FS
                    ']' => Some(vec![0x1d]),     // Ctrl+]: GS
                    '^' => Some(vec![0x1e]),     // Ctrl+^: RS
                    '_' => Some(vec![0x1f]),     // Ctrl+_: US
                    '?' => Some(vec![0x7f]),     // Ctrl+?: DEL
                    // For other control characters, send as-is
                    _ => Some(c.to_string().into_bytes())
                }
            } else if key.modifiers.contains(KeyModifiers::ALT) {
                // Alt sends ESC followed by the character
                let mut bytes = vec![0x1b]; // ESC
                bytes.extend(c.to_string().as_bytes());
                Some(bytes)
            } else {
                Some(c.to_string().into_bytes())
            }
        }
        KeyCode::Enter => Some(vec![b'\r']),
        KeyCode::Backspace => Some(vec![0x7f]), // DEL
        KeyCode::Delete => Some(vec![0x1b, b'[', b'3', b'~']),
        KeyCode::Left => Some(vec![0x1b, b'[', b'D']),
        KeyCode::Right => Some(vec![0x1b, b'[', b'C']),
        KeyCode::Up => Some(vec![0x1b, b'[', b'A']),
        KeyCode::Down => Some(vec![0x1b, b'[', b'B']),
        KeyCode::Home => Some(vec![0x1b, b'[', b'H']),
        KeyCode::End => Some(vec![0x1b, b'[', b'F']),
        KeyCode::PageUp => Some(vec![0x1b, b'[', b'5', b'~']),
        KeyCode::PageDown => Some(vec![0x1b, b'[', b'6', b'~']),
        KeyCode::Tab => Some(vec![b'\t']),
        KeyCode::BackTab => Some(vec![0x1b, b'[', b'Z']),
        KeyCode::Esc => Some(vec![0x1b]),
        KeyCode::Insert => Some(vec![0x1b, b'[', b'2', b'~']),
        KeyCode::F(n) => {
            // F1-F12 function keys
            match n {
                1 => Some(vec![0x1b, b'O', b'P']),
                2 => Some(vec![0x1b, b'O', b'Q']),
                3 => Some(vec![0x1b, b'O', b'R']),
                4 => Some(vec![0x1b, b'O', b'S']),
                5 => Some(vec![0x1b, b'[', b'1', b'5', b'~']),
                6 => Some(vec![0x1b, b'[', b'1', b'7', b'~']),
                7 => Some(vec![0x1b, b'[', b'1', b'8', b'~']),
                8 => Some(vec![0x1b, b'[', b'1', b'9', b'~']),
                9 => Some(vec![0x1b, b'[', b'2', b'0', b'~']),
                10 => Some(vec![0x1b, b'[', b'2', b'1', b'~']),
                11 => Some(vec![0x1b, b'[', b'2', b'3', b'~']),
                12 => Some(vec![0x1b, b'[', b'2', b'4', b'~']),
                _ => None,
            }
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new().await?;

    // Run the application
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Create main layout with tabs at top and content below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Help bar
        ])
        .split(size);

    // Render tabs
    let tab_titles: Vec<String> = app
        .sessions
        .iter()
        .map(|s| s.name.clone())
        .collect();
    
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("TUI Sessions"))
        .select(app.selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[0]);

    // Render selected session content
    if let Some(session) = app.sessions.get(app.selected_tab) {
        render_session(f, session, chunks[1]);
    }

    // Render help bar
    let help_text = vec![Line::from(vec![
        Span::styled("Switch: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("Ctrl+← →  "),
        Span::styled("Jump: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("Ctrl+1-9  "),
        Span::styled("Quit: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("Ctrl+Q/C  "),
        Span::styled("Input: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("Forwarded to active TUI"),
    ])];

    let help = Paragraph::new(help_text).style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[2]);
}

fn render_session(f: &mut Frame, session: &TuiSession, area: Rect) {
    let output = session.get_output();
    
    // Split output into lines and take the last N lines that fit in the area
    let lines: Vec<&str> = output.lines().collect();
    let available_lines = (area.height.saturating_sub(2)) as usize;
    let start_line = if lines.len() > available_lines {
        lines.len() - available_lines
    } else {
        0
    };
    
    let display_lines: Vec<Line> = lines[start_line..]
        .iter()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let title = format!(" {} - {} ", session.name, session.command);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));

    let paragraph = Paragraph::new(display_lines)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}
