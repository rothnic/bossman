# Bossman Demo Screenshots

This directory contains ASCII art mockups demonstrating key use cases of the Bossman terminal multiplexer.

## Screenshots Overview

### 1. Initial Launch (`01-initial-launch.txt`)
- **State**: Application just started
- **Active Tab**: Watch Date (first tab)
- **Shows**: Basic layout with 4 tabs and help text at bottom
- **Demonstrates**: Clean UI structure, tab bar, content area

### 2. Switching to System Monitor (`02-switch-to-system-monitor.txt`)
- **Action**: User pressed Ctrl+→ to switch tabs
- **Active Tab**: System Monitor (second tab)
- **Shows**: Live system metrics from `top` command
- **Demonstrates**: Tab navigation, live output capture from running process

### 3. Log Viewer with Live Updates (`03-log-viewer-streaming.txt`)
- **Action**: User pressed Ctrl+→ or Ctrl+3
- **Active Tab**: Log Viewer (third tab)
- **Shows**: Streaming log output with timestamps
- **Demonstrates**: Real-time output buffering, automatic buffer management

### 4. Interactive Shell - Typing Command (`04-shell-typing-command.txt`)
- **Action**: User pressed Ctrl+4, then typed a command
- **Active Tab**: Shell (fourth tab)
- **Shows**: Bash shell with command history and user typing
- **Demonstrates**: Input forwarding - user can type into the shell
- **Key Feature**: Cursor position visible, command being typed

### 5. Shell - Command Executed (`05-shell-command-executed.txt`)
- **Action**: User pressed Enter to execute command
- **Active Tab**: Shell (still active)
- **Shows**: Command output and new prompt
- **Demonstrates**: Bidirectional I/O - input forwarded, output captured
- **Key Feature**: Enter key properly forwarded as 0x0D byte

### 6. OpenCode Slash Command Menu (`06-opencode-slash-menu.txt`)
- **Action**: User configured OpenCode session, typed "/" 
- **Active Tab**: OpenCode Main
- **Shows**: OpenCode TUI with slash menu open
- **Demonstrates**: Core use case - interacting with OpenCode
- **Key Features**:
  - Slash command (/) forwarded and menu opened
  - Arrow keys work for menu navigation
  - Can select commands and enter text

### 7. Tab Completion Working (BONUS) (`07-tab-completion-working.txt`)
- **Action**: User typed partial filename, pressed Tab
- **Active Tab**: Shell
- **Shows**: Tab completion working in bash
- **Demonstrates**: Critical fix - Tab key forwarded to TUI
- **Key Feature**: Tab/Shift+Tab no longer intercepted for navigation
- **Importance**: Shows the improvement from fixing key conflicts

## Key Features Demonstrated

### Navigation
- **Ctrl+Left/Right**: Switch between tabs (Screenshots 1-3)
- **Ctrl+1-9**: Jump directly to specific tab (Screenshot 4)
- **Visual Feedback**: Active tab highlighted in cyan with bold text

### Input Forwarding
- **Typing**: All characters forwarded to active session (Screenshot 4)
- **Special Keys**: Enter, arrows, etc. work properly (Screenshots 5-6)
- **Tab Completion**: Tab/Shift+Tab forwarded for completion (Screenshot 7)
- **Slash Commands**: Forward all keys including "/" (Screenshot 6)

### Output Capture
- **Real-time**: Output appears immediately (Screenshots 2-3, 5)
- **Buffering**: Automatic buffer management (100KB limit)
- **Multiple Sessions**: All sessions capture output concurrently

### Use Cases Covered

1. **System Monitoring**: View system metrics (Screenshot 2)
2. **Log Analysis**: Monitor streaming logs (Screenshot 3)
3. **Command Execution**: Run shell commands (Screenshots 4-5)
4. **Tab Completion**: Use bash completion (Screenshot 7)
5. **OpenCode Interaction**: Work with coding agent TUI (Screenshot 6)

## How to View

These are text-based mockups. View them with any text editor or cat command:

```bash
cd screenshots
cat 01-initial-launch.txt
cat 02-switch-to-system-monitor.txt
cat 03-log-viewer-streaming.txt
cat 04-shell-typing-command.txt
cat 05-shell-command-executed.txt
cat 06-opencode-slash-menu.txt
cat 07-tab-completion-working.txt
```

Or view all at once:
```bash
for f in screenshots/*.txt; do echo; cat "$f"; echo; done
```

## Technical Notes

- All screenshots use UTF-8 box drawing characters
- Cursor represented by █ character
- Active tab marked with * in tab title
- Cyan highlighting indicates active selection
- Help text always visible at bottom

## Actual Application

These mockups accurately represent the actual application behavior:
- Tab switching works as shown
- Input forwarding functions exactly as demonstrated
- All keys are properly converted to PTY byte sequences
- Real TUI applications (OpenCode, vim, htop) work as depicted
