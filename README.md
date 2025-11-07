# bossman
A TUI for monitoring and managing multiple coding agent sessions

## Overview
Bossman is a terminal user interface (TUI) application built with Ratatui for managing multiple coding agent sessions simultaneously. The MVP provides a 4-panel grid layout for monitoring and controlling agent sessions.

## Features
- **4-Panel Grid Layout**: Monitor up to 4 agent sessions simultaneously in a 2x2 grid
- **Interactive Navigation**: Navigate between sessions using arrow keys or vim-style (hjkl) keybindings
- **Session Management**: Start, stop, and monitor individual agent sessions
- **Visual Feedback**: Color-coded status indicators (Idle: Yellow, Running: Green, Stopped: Red)
- **Real-time Updates**: View session output in real-time within each panel

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
cargo run
# or
./target/release/bossman
```

### Keyboard Controls
- **Navigation**: 
  - Arrow keys (←↑↓→) or vim-style (h, j, k, l)
  - Navigate between the 4 agent panels
- **Session Control**:
  - `s` - Toggle session status (Idle → Running → Stopped)
- **Application**:
  - `q` - Quit the application

### Current Behavior
The MVP displays 4 agent session panels in a grid. Each panel shows:
- Agent name and status
- Session output (most recent messages)
- Visual selection indicator (cyan border for selected panel)

Press `s` on any panel to cycle through session states:
1. **Idle** (Yellow) - Session initialized but not started
2. **Running** (Green) - Session is active
3. **Stopped** (Red) - Session has been stopped

## Architecture

### Components
- **App State**: Manages overall application state and session data
- **AgentSession**: Represents individual agent sessions with status and output
- **UI Renderer**: Handles terminal rendering with Ratatui
- **Event Handler**: Processes keyboard input for navigation and control

### Future Enhancements
- Async agent execution and management
- Real process/command integration
- Session persistence and recovery
- Configurable number of panels
- Log file support
- Network-based agent communication

## Dependencies
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## License
MIT
