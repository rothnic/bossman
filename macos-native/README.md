# Bossman Native macOS App

This directory contains a native macOS application implementation of Bossman using Swift and AppKit.

## Overview

The native macOS version provides the same functionality as the terminal-based version but with native macOS UI:

- Native window management
- NSTabView for tab interface
- NSTextView for terminal output display
- Direct PTY integration using macOS APIs

## Features

- **Native macOS UI**: Uses AppKit for native look and feel
- **Tab Interface**: NSTabView for managing multiple sessions
- **PTY Support**: Direct use of macOS PTY APIs (`openpty`)
- **Real-time Output**: Background thread reads from PTY and updates UI

## Building

### Prerequisites
- macOS 12.0 or later
- Xcode 14.0 or later
- Swift 5.7 or later

### Build and Run

```bash
cd macos-native
swift build
swift run
```

Or open in Xcode:
```bash
swift package generate-xcodeproj
open BossmanNative.xcodeproj
```

## Architecture

### Components

1. **AppDelegate**: Application lifecycle management
2. **MainViewController**: Manages the tab view and sessions
3. **TUISession**: Represents a single TUI application with PTY
4. **TerminalViewController**: Displays terminal output in NSTextView

### How PTY Works

The native app uses macOS's `openpty()` function to create a pseudo-terminal pair:

```swift
var masterFD: Int32 = -1
var slaveFD: Int32 = -1
openpty(&masterFD, &slaveFD, nil, nil, nil)
```

The slave end is connected to the spawned process's stdin/stdout/stderr, while the master end is read in a background thread to capture output.

## Comparison with Terminal Version

| Feature | Terminal Version | Native macOS |
|---------|-----------------|--------------|
| UI Framework | Ratatui/TUI | AppKit |
| Platform | Cross-platform | macOS only |
| Window Management | Terminal emulator | Native windows |
| PTY Library | portable-pty | macOS APIs |
| Build System | Cargo | Swift Package Manager |

## Advantages of Native Version

1. **Better Integration**: Native macOS keyboard shortcuts and window management
2. **Multiple Windows**: Can open multiple windows with different session sets
3. **System Theme**: Follows macOS appearance settings
4. **Accessibility**: Better support for macOS accessibility features

## Future Enhancements

- [ ] Input forwarding to PTY (currently read-only)
- [ ] Configurable session list
- [ ] Session persistence and restoration
- [ ] ANSI color code rendering
- [ ] Copy/paste support
- [ ] Search functionality
- [ ] Split panes in addition to tabs

## Limitations

- Currently output-only (no input to child processes)
- Basic ANSI escape code handling
- No color rendering yet
- Requires macOS 12.0+

## License

MIT
