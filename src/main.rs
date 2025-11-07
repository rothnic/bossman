use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

/// Represents a single agent session
#[derive(Debug, Clone)]
struct AgentSession {
    #[allow(dead_code)]
    id: usize,
    name: String,
    status: SessionStatus,
    output: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum SessionStatus {
    Idle,
    Running,
    Stopped,
}

impl AgentSession {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            status: SessionStatus::Idle,
            output: vec![
                format!("Session {} initialized", id),
                "Ready to start...".to_string(),
            ],
        }
    }
}

/// Main application state
struct App {
    sessions: Vec<AgentSession>,
    selected_session: usize,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let sessions = vec![
            AgentSession::new(0, "Agent 1".to_string()),
            AgentSession::new(1, "Agent 2".to_string()),
            AgentSession::new(2, "Agent 3".to_string()),
            AgentSession::new(3, "Agent 4".to_string()),
        ];

        Self {
            sessions,
            selected_session: 0,
            should_quit: false,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Left | KeyCode::Char('h') => {
                if self.selected_session % 2 == 1 {
                    self.selected_session -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.selected_session % 2 == 0 && self.selected_session < 3 {
                    self.selected_session += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_session >= 2 {
                    self.selected_session -= 2;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected_session < 2 {
                    self.selected_session += 2;
                }
            }
            KeyCode::Char('s') => {
                // Toggle session status
                let session = &mut self.sessions[self.selected_session];
                session.status = match session.status {
                    SessionStatus::Idle => {
                        session.output.push(format!("Starting {}...", session.name));
                        SessionStatus::Running
                    }
                    SessionStatus::Running => {
                        session.output.push(format!("{} stopped", session.name));
                        SessionStatus::Stopped
                    }
                    SessionStatus::Stopped => {
                        session.output.push(format!("{} restarting...", session.name));
                        SessionStatus::Running
                    }
                };
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the application
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    // Create a 2x2 grid layout
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(size);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(vertical_chunks[1]);

    let chunks = vec![
        top_chunks[0],
        top_chunks[1],
        bottom_chunks[0],
        bottom_chunks[1],
    ];

    // Render each session in its panel
    for (idx, chunk) in chunks.iter().enumerate() {
        render_session(f, app, idx, *chunk);
    }

    // Render help text at the bottom
    let help_text = vec![
        Line::from(vec![
            Span::styled("Navigation: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("←↑↓→ or hjkl  "),
            Span::styled("Toggle: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("s  "),
            Span::styled("Quit: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("q"),
        ]),
    ];

    let help_paragraph = Paragraph::new(help_text).style(Style::default().fg(Color::Gray));
    
    // Create a small area at the bottom for help text
    let help_area = Rect {
        x: 0,
        y: size.height.saturating_sub(1),
        width: size.width,
        height: 1,
    };
    
    f.render_widget(help_paragraph, help_area);
}

fn render_session(f: &mut ratatui::Frame, app: &App, idx: usize, area: Rect) {
    let session = &app.sessions[idx];
    let is_selected = idx == app.selected_session;

    let (_border_color, border_style) = if is_selected {
        (Color::Cyan, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    } else {
        (Color::Gray, Style::default().fg(Color::Gray))
    };

    let status_color = match session.status {
        SessionStatus::Idle => Color::Yellow,
        SessionStatus::Running => Color::Green,
        SessionStatus::Stopped => Color::Red,
    };

    let title = format!(
        " {} [{:?}] ",
        session.name,
        session.status
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(title)
        .title_style(Style::default().fg(status_color).add_modifier(Modifier::BOLD));

    // Display session output
    let output_lines: Vec<Line> = session
        .output
        .iter()
        .rev()
        .take(area.height.saturating_sub(2) as usize)
        .rev()
        .map(|line| Line::from(line.clone()))
        .collect();

    let paragraph = Paragraph::new(output_lines)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}
