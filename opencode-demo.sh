#!/bin/bash
# Example script showing how to run Bossman with custom OpenCode sessions

# This script demonstrates the concept - you would actually modify src/main.rs
# to add these sessions, or use a configuration file approach (future feature)

cat << 'EOF'
╔════════════════════════════════════════════════════════════════╗
║              Bossman - OpenCode Integration Demo              ║
╚════════════════════════════════════════════════════════════════╝

This is a demonstration of how Bossman can be used to run multiple
OpenCode instances (or other TUI agents) simultaneously.

Example Configuration:
─────────────────────

Tab 1: OpenCode - Main Project
  Command: opencode
  Args: --workspace /path/to/main-project
  Purpose: Primary development work

Tab 2: OpenCode - Testing
  Command: opencode  
  Args: --workspace /path/to/test-suite
  Purpose: Test development and debugging

Tab 3: OpenCode - Documentation
  Command: opencode
  Args: --workspace /path/to/docs
  Purpose: Documentation updates

Tab 4: System Monitor
  Command: htop
  Purpose: Resource monitoring

Key Features:
─────────────

✓ Each OpenCode instance runs independently in its own PTY
✓ Switch between agents using Tab/Shift+Tab
✓ Jump to specific agent with Ctrl+1, Ctrl+2, etc.
✓ All agents run concurrently
✓ Real-time output from all sessions
✓ No nested TUI complexity

Usage Instructions:
──────────────────

1. Edit src/main.rs and modify the App::new() function
2. Replace the example sessions with your OpenCode commands
3. Build: cargo build --release
4. Run: cargo run --release

Example Code Modification:
─────────────────────────

impl App {
    async fn new() -> Result<Self> {
        let sessions = vec![
            TuiSession::new(
                "OpenCode - Main".to_string(),
                "opencode".to_string(),
                vec!["--workspace".to_string(), "/path/to/project".to_string()],
            )?,
            // Add more sessions as needed
        ];
        
        Ok(Self {
            sessions,
            selected_tab: 0,
            should_quit: false,
        })
    }
}

See OPENCODE_INTEGRATION.md for detailed instructions.

EOF
