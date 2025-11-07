# Integrating OpenCode with Bossman

This guide shows how to integrate OpenCode (or any other TUI application) with Bossman.

## Prerequisites

- Bossman installed and working
- OpenCode installed and accessible in PATH

## Quick Start

### Option 1: Modify the Default Sessions

Edit `src/main.rs` and replace one of the default sessions with OpenCode:

```rust
impl App {
    async fn new() -> Result<Self> {
        let sessions = vec![
            // OpenCode session
            TuiSession::new(
                "OpenCode".to_string(),
                "opencode".to_string(),
                vec![],  // Add any OpenCode arguments here
            )?,
            // Keep other sessions or add more TUI applications
            TuiSession::new(
                "System Monitor".to_string(),
                "htop".to_string(),
                vec![],
            )?,
            TuiSession::new(
                "File Manager".to_string(),
                "ranger".to_string(),
                vec![],
            )?,
            TuiSession::new(
                "Text Editor".to_string(),
                "vim".to_string(),
                vec![],
            )?,
        ];

        Ok(Self {
            sessions,
            selected_tab: 0,
            should_quit: false,
        })
    }
}
```

### Option 2: Create a Configuration File

For a more flexible approach, create a configuration-based session loader (future enhancement):

```toml
# bossman.toml
[[sessions]]
name = "OpenCode"
command = "opencode"
args = []

[[sessions]]
name = "Agent 2"
command = "another-tui-agent"
args = ["--config", "agent2.json"]

[[sessions]]
name = "System Monitor"
command = "htop"
args = []
```

## Running Multiple OpenCode Instances

You can run multiple OpenCode instances with different configurations:

```rust
let sessions = vec![
    TuiSession::new(
        "OpenCode - Project A".to_string(),
        "opencode".to_string(),
        vec!["--workspace".to_string(), "/path/to/project-a".to_string()],
    )?,
    TuiSession::new(
        "OpenCode - Project B".to_string(),
        "opencode".to_string(),
        vec!["--workspace".to_string(), "/path/to/project-b".to_string()],
    )?,
];
```

## Environment Variables

If OpenCode requires specific environment variables, you can extend the `TuiSession::new()` method to support them:

```rust
impl TuiSession {
    fn new_with_env(
        name: String, 
        command: String, 
        args: Vec<String>,
        env_vars: Vec<(String, String)>
    ) -> Result<Self> {
        // ... existing PTY setup code ...
        
        let mut cmd = CommandBuilder::new(&command);
        for arg in args {
            cmd.arg(arg);
        }
        
        // Add environment variables
        for (key, value) in env_vars {
            cmd.env(key, value);
        }
        
        // ... rest of the code ...
    }
}
```

Then use it like:

```rust
TuiSession::new_with_env(
    "OpenCode".to_string(),
    "opencode".to_string(),
    vec![],
    vec![
        ("OPENCODE_API_KEY".to_string(), "your-key-here".to_string()),
        ("TERM".to_string(), "xterm-256color".to_string()),
    ],
)?
```

## Troubleshooting

### OpenCode Not Responding

If OpenCode appears frozen or doesn't respond to input:

1. Ensure the PTY size is appropriate:
   ```rust
   let pair = pty_system.openpty(PtySize {
       rows: 40,    // Increase if needed
       cols: 120,   // Increase if needed
       pixel_width: 0,
       pixel_height: 0,
   })?;
   ```

2. Check that the TERM environment variable is set:
   ```rust
   cmd.env("TERM", "xterm-256color");
   ```

### ANSI Escape Codes Not Rendering

The current implementation displays raw output. For full ANSI color support, you would need to:

1. Parse ANSI escape codes from the output
2. Convert them to Ratatui styles
3. Apply the styles when rendering

This is a planned enhancement for a future version.

## Using Input with OpenCode

Input forwarding is now fully implemented! You can:

1. **Type directly** into the active session - all keystrokes are forwarded
2. **Use arrow keys** to navigate within OpenCode
3. **Press Enter** to confirm commands
4. **Use slash commands** (/) to access OpenCode features
5. **Tab completion** works within OpenCode itself

The following keys are reserved for Bossman controls and won't be forwarded:
- `Ctrl+Q` or `Ctrl+C` - Quit Bossman
- `Ctrl+Left/Right` - Switch tabs
- `Ctrl+1` through `Ctrl+9` - Jump to specific tabs

All other keys (including Tab and Shift+Tab) are forwarded to the active TUI session.

## Best Practices

1. **Resource Management**: Each TUI session consumes system resources. Limit the number of concurrent sessions based on your system capabilities.

2. **Session Naming**: Use descriptive names to easily identify each session.

3. **Error Handling**: Monitor the session status and handle failures gracefully.

4. **Cleanup**: Sessions are automatically cleaned up when Bossman exits, but you can add explicit cleanup if needed.

## Example: Full Multi-Agent Setup

Here's a complete example running multiple coding agents:

```rust
impl App {
    async fn new() -> Result<Self> {
        let sessions = vec![
            // Primary coding agent
            TuiSession::new(
                "OpenCode - Main".to_string(),
                "opencode".to_string(),
                vec!["--workspace".to_string(), "./".to_string()],
            )?,
            
            // Secondary agent for different task
            TuiSession::new(
                "OpenCode - Tests".to_string(),
                "opencode".to_string(),
                vec!["--workspace".to_string(), "./tests".to_string()],
            )?,
            
            // Monitoring tools
            TuiSession::new(
                "System Resources".to_string(),
                "htop".to_string(),
                vec![],
            )?,
            
            // Logs viewer
            TuiSession::new(
                "Build Logs".to_string(),
                "tail".to_string(),
                vec!["-f".to_string(), "build.log".to_string()],
            )?,
        ];

        Ok(Self {
            sessions,
            selected_tab: 0,
            should_quit: false,
        })
    }
}
```

## Next Steps

1. Build Bossman with your custom configuration
2. Run and test the OpenCode integration
3. Adjust PTY size and settings as needed
4. Add more sessions for other tools or agents

For more information, see the main README.md file.
