# Bossman 🎯

A web-based terminal manager for monitoring and managing multiple coding agent sessions.

## Overview

Bossman provides a web interface with 4 interactive terminal sessions running in a grid layout. Each terminal is a fully functional shell that can run any command-line tool, including AI coding agents like OpenCode.

## Why Web-Based?

The original TUI (Terminal User Interface) approach had limitations when trying to embed other agent TUIs within it. A web-based solution with terminal emulation solves this by:

- Running each agent in its own isolated pseudo-terminal (PTY)
- Providing full terminal emulation in the browser via xterm.js
- Allowing independent interaction with each terminal session
- Supporting any CLI tool without nesting issues

## Features

- **4 Terminal Grid**: 2x2 grid layout with independent terminal sessions
- **Full Terminal Emulation**: Powered by xterm.js with complete ANSI escape sequence support
- **Real-time Communication**: WebSocket-based for instant terminal I/O
- **Independent Sessions**: Each terminal runs in its own PTY with isolated state
- **Easy Management**: Kill and restart individual terminals as needed

## Tech Stack

- **Backend**: Node.js + Express + Socket.IO + node-pty
- **Frontend**: HTML5 + xterm.js + Socket.IO client
- **Terminal Emulation**: xterm.js with fit addon for responsive sizing

## Installation

1. **Prerequisites**: Node.js 18+ and npm

2. **Install dependencies**:
```bash
npm install
```

## Usage

1. **Start the server**:
```bash
npm start
```

2. **Open your browser**:
Navigate to `http://localhost:3000`

3. **Use the terminals**:
- Each terminal starts with a bash/PowerShell prompt
- Type commands as you would in a regular terminal
- Use Ctrl+C to interrupt running processes
- Click the ✕ button to kill a specific terminal
- Refresh the page to restart all terminals

## Running OpenCode (or other agents)

To run OpenCode in any terminal:

```bash
# If OpenCode is not installed, you can typically install it via pip
pip install opencode

# Run OpenCode with built-in Zen models (no authentication needed)
opencode --model zen
```

Each terminal can run a different agent or task simultaneously:
- Terminal 1: OpenCode agent working on feature A
- Terminal 2: OpenCode agent working on feature B
- Terminal 3: Running tests or monitoring
- Terminal 4: Manual debugging or inspection

## Architecture

```
┌─────────────────────────────────────────┐
│         Web Browser                     │
│  ┌────────────┬────────────┐            │
│  │ xterm.js 1 │ xterm.js 2 │            │
│  ├────────────┼────────────┤            │
│  │ xterm.js 3 │ xterm.js 4 │            │
│  └────────────┴────────────┘            │
└──────────────┬──────────────────────────┘
               │ WebSocket (Socket.IO)
┌──────────────▼──────────────────────────┐
│      Express + Socket.IO Server         │
│  ┌────────────┬────────────┐            │
│  │  PTY 1     │  PTY 2     │            │
│  ├────────────┼────────────┤            │
│  │  PTY 3     │  PTY 4     │            │
│  └────────────┴────────────┘            │
└─────────────────────────────────────────┘
```

## Development

The server runs on port 3000 by default. You can change this by setting the `PORT` environment variable:

```bash
PORT=8080 npm start
```

## Future Enhancements

- Configurable number of terminals
- Save and restore terminal sessions
- Terminal tabs for more sessions
- Agent-specific presets (auto-start commands)
- Session persistence across page refreshes
- Log recording and playback
