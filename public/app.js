// Connect to Socket.IO server
const socket = io();

// Store terminal instances
const terminals = {};
const fitAddons = {};

// Terminal IDs
const terminalIds = ['terminal-1', 'terminal-2', 'terminal-3', 'terminal-4'];

// Initialize terminals
function initializeTerminals() {
    terminalIds.forEach((terminalId) => {
        createTerminal(terminalId);
    });
}

function createTerminal(terminalId) {
    const container = document.getElementById(terminalId);
    if (!container) {
        console.error(`Container not found: ${terminalId}`);
        return;
    }

    // Create xterm instance
    const term = new Terminal({
        cursorBlink: true,
        fontSize: 14,
        fontFamily: 'Menlo, Monaco, "Courier New", monospace',
        theme: {
            background: '#1e1e1e',
            foreground: '#cccccc',
            cursor: '#4ec9b0',
            black: '#000000',
            red: '#cd3131',
            green: '#0dbc79',
            yellow: '#e5e510',
            blue: '#2472c8',
            magenta: '#bc3fbc',
            cyan: '#11a8cd',
            white: '#e5e5e5',
            brightBlack: '#666666',
            brightRed: '#f14c4c',
            brightGreen: '#23d18b',
            brightYellow: '#f5f543',
            brightBlue: '#3b8eea',
            brightMagenta: '#d670d6',
            brightCyan: '#29b8db',
            brightWhite: '#ffffff'
        }
    });

    // Create fit addon
    const fitAddon = new FitAddon.FitAddon();
    term.loadAddon(fitAddon);

    // Open terminal in container
    term.open(container);
    
    // Fit terminal to container
    setTimeout(() => {
        fitAddon.fit();
    }, 0);

    // Store instances
    terminals[terminalId] = term;
    fitAddons[terminalId] = fitAddon;

    // Request server to create PTY
    socket.emit('create-terminal', terminalId);

    // Handle user input
    term.onData((data) => {
        socket.emit('terminal-input', { terminalId, data });
    });

    // Listen for output from server
    socket.on(`terminal-output-${terminalId}`, (data) => {
        term.write(data);
    });

    // Handle terminal exit
    socket.on(`terminal-exit-${terminalId}`, ({ exitCode }) => {
        term.write(`\r\n\r\n*** Terminal exited with code ${exitCode} ***\r\n`);
        term.write('*** Refresh page to restart ***\r\n');
    });

    // Handle resize
    const resizeObserver = new ResizeObserver(() => {
        fitAddon.fit();
        socket.emit('terminal-resize', {
            terminalId,
            cols: term.cols,
            rows: term.rows
        });
    });
    resizeObserver.observe(container);
}

function killTerminal(terminalId) {
    if (terminals[terminalId]) {
        socket.emit('kill-terminal', terminalId);
        terminals[terminalId].write('\r\n\r\n*** Terminal killed ***\r\n');
        terminals[terminalId].write('*** Refresh page to restart ***\r\n');
    }
}

// Handle window resize
window.addEventListener('resize', () => {
    Object.keys(fitAddons).forEach((terminalId) => {
        fitAddons[terminalId].fit();
        const term = terminals[terminalId];
        socket.emit('terminal-resize', {
            terminalId,
            cols: term.cols,
            rows: term.rows
        });
    });
});

// Initialize when page loads
window.addEventListener('load', () => {
    initializeTerminals();
});
