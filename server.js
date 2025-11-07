const express = require('express');
const http = require('http');
const socketIO = require('socket.io');
const pty = require('node-pty');
const path = require('path');
const os = require('os');

const app = express();
const server = http.createServer(app);
const io = socketIO(server);

const PORT = process.env.PORT || 3000;

// Serve static files from the public directory
app.use(express.static(path.join(__dirname, 'public')));

// Serve xterm.js library files
app.use('/xterm', express.static(path.join(__dirname, 'node_modules/xterm')));
app.use('/xterm-addon-fit', express.static(path.join(__dirname, 'node_modules/xterm-addon-fit')));

// Store active terminal sessions
const terminals = {};
const logs = {};

// Determine shell based on OS
const shell = os.platform() === 'win32' ? 'powershell.exe' : 'bash';

io.on('connection', (socket) => {
  console.log('Client connected:', socket.id);

  // Create a new terminal session
  socket.on('create-terminal', (terminalId) => {
    if (terminals[terminalId]) {
      console.log(`Terminal ${terminalId} already exists`);
      return;
    }

    console.log(`Creating terminal: ${terminalId}`);
    
    // Spawn a new pseudo-terminal
    const term = pty.spawn(shell, [], {
      name: 'xterm-color',
      cols: 80,
      rows: 24,
      cwd: process.env.HOME || process.cwd(),
      env: process.env
    });

    terminals[terminalId] = term;
    logs[terminalId] = [];

    // Send output from PTY to client
    term.onData((data) => {
      logs[terminalId].push(data);
      io.emit(`terminal-output-${terminalId}`, data);
    });

    // Handle terminal exit
    term.onExit(({ exitCode, signal }) => {
      console.log(`Terminal ${terminalId} exited with code ${exitCode}`);
      delete terminals[terminalId];
      io.emit(`terminal-exit-${terminalId}`, { exitCode, signal });
    });

    // Send initial prompt
    socket.emit(`terminal-output-${terminalId}`, '\r\n*** Terminal Ready ***\r\n\r\n');
  });

  // Handle input from client
  socket.on('terminal-input', ({ terminalId, data }) => {
    if (terminals[terminalId]) {
      terminals[terminalId].write(data);
    }
  });

  // Resize terminal
  socket.on('terminal-resize', ({ terminalId, cols, rows }) => {
    if (terminals[terminalId]) {
      terminals[terminalId].resize(cols, rows);
    }
  });

  // Kill terminal
  socket.on('kill-terminal', (terminalId) => {
    if (terminals[terminalId]) {
      terminals[terminalId].kill();
      delete terminals[terminalId];
      delete logs[terminalId];
    }
  });

  socket.on('disconnect', () => {
    console.log('Client disconnected:', socket.id);
  });
});

server.listen(PORT, () => {
  console.log(`Bossman server running on http://localhost:${PORT}`);
  console.log(`Open your browser and navigate to the URL above`);
});
