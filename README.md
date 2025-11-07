# bossman

A terminal multiplexer for running multiple TUI applications with tab switching

## Overview

Bossman is a terminal-based application that allows you to run multiple TUI (Text User Interface) applications simultaneously in separate sessions. Each session runs in its own pseudo-terminal (PTY), and you can easily switch between them using keyboard shortcuts.

This solves the problem of wanting to run multiple agent TUIs (like OpenCode, vim, htop, etc.) in a single interface without the complexity of nested TUI implementations.

## Features

- **Multiple TUI Sessions**: Run different TUI applications in parallel
- **Tab-Based Switching**: Easy navigation between sessions using Tab or Ctrl+Arrow keys
- **Real PTY Support**: Each session runs in its own pseudo-terminal for full compatibility
- **Live Output**: See real-time output from all running sessions
- **Simple Configuration**: Easily customize which applications to run

## Installation

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build from Source
```bash
git clone https://github.com/rothnic/bossman.git
cd bossman
cargo build --release
```

## Usage

### Running Bossman
```bash
cargo run --release
# or
./target/release/bossman
```

### Keyboard Controls

- **Tab** or **Ctrl+→**: Switch to next tab
- **Shift+Tab** or **Ctrl+←**: Switch to previous tab
- **Ctrl+1** through **Ctrl+9**: Jump directly to tab 1-9
- **Ctrl+Q** or **Ctrl+C**: Quit the application

### Default Sessions

By default, Bossman launches with 4 example sessions:
1. **Watch Date**: A simple date watcher showing live updates
2. **System Monitor**: The `top` command showing system resources
3. **Log Viewer**: A simulated log stream with timestamps
4. **Shell**: An interactive bash shell

### Customizing Sessions

To customize which applications run, modify the `App::new()` function in `src/main.rs`:

```rust
sessions.push(TuiSession::new(
    "My App".to_string(),        // Display name
    "command".to_string(),        // Command to run
    vec!["arg1".to_string()],     // Arguments
)?);
```

### Example: Running OpenCode

To run OpenCode (or any other TUI application) in a session:

```rust
sessions.push(TuiSession::new(
    "OpenCode".to_string(),
    "opencode".to_string(),
    vec![],
)?);
```

## Architecture

### Components

- **TuiSession**: Manages a single TUI application with its own PTY
- **App**: Main application state managing multiple sessions
- **UI Renderer**: Ratatui-based rendering with tabs and content areas
- **PTY System**: Uses `portable-pty` for cross-platform pseudo-terminal support

### How It Works

1. Each session creates a pseudo-terminal (PTY) pair
2. The target application is spawned in the PTY slave
3. Output is captured from the PTY master in a background task
4. The main UI displays the current tab's output buffer
5. Input events control tab switching and application lifecycle

## Platform Support

- **Linux**: ✅ Full support
- **macOS**: ✅ Full support (native PTY)
- **Windows**: ⚠️ Limited (requires WSL or similar)

## macOS Native App

For a true native macOS experience with native window management, a Swift/AppKit version could be created that uses:
- `NSTabView` for tab management
- `NSTextView` or custom views for terminal rendering
- Native macOS PTY APIs

The current Rust implementation works well on macOS but runs in the terminal. A native app would provide:
- Native window chrome and controls
- macOS-specific keyboard shortcuts
- Better integration with system theme
- Multi-window support

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [portable-pty](https://github.com/wez/wezterm/tree/main/pty) - Cross-platform PTY support
- [tokio](https://tokio.rs/) - Async runtime for background tasks
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling

## Related Projects

This approach is inspired by:
- [tmux](https://github.com/tmux/tmux) - Terminal multiplexer
- [screen](https://www.gnu.org/software/screen/) - Terminal multiplexer
- [zellij](https://github.com/zellij-org/zellij) - Modern terminal workspace

## Troubleshooting

### PTY Issues
If you encounter PTY-related errors, ensure your system has proper PTY support:
```bash
# Check PTY availability on Linux
ls -la /dev/ptmx
```

### Application Not Responding
Some applications may need specific terminal environment variables:
```rust
cmd.env("TERM", "xterm-256color");
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
