import AppKit
import Foundation

/// Main application entry point
@main
class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    var mainViewController: MainViewController!
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        // Create main window
        window = NSWindow(
            contentRect: NSRect(x: 100, y: 100, width: 1200, height: 800),
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        )
        
        window.title = "Bossman - TUI Multiplexer"
        window.center()
        
        // Create and set main view controller
        mainViewController = MainViewController()
        window.contentViewController = mainViewController
        
        window.makeKeyAndOrderFront(nil)
        
        NSApp.activate(ignoringOtherApps: true)
    }
    
    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
}

/// Main view controller managing the tab interface
class MainViewController: NSViewController {
    var tabView: NSTabView!
    var sessions: [TUISession] = []
    
    override func loadView() {
        view = NSView(frame: NSRect(x: 0, y: 0, width: 1200, height: 800))
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        setupTabView()
        createDefaultSessions()
    }
    
    private func setupTabView() {
        tabView = NSTabView(frame: view.bounds)
        tabView.autoresizingMask = [.width, .height]
        tabView.tabViewType = .topTabsBezelBorder
        view.addSubview(tabView)
    }
    
    private func createDefaultSessions() {
        // Create example sessions
        addSession(name: "Watch Date", command: "watch", args: ["-n", "1", "date"])
        addSession(name: "System Monitor", command: "top", args: [])
        addSession(name: "Log Viewer", command: "tail", args: ["-f", "/var/log/system.log"])
        addSession(name: "Shell", command: "/bin/bash", args: ["--login"])
    }
    
    private func addSession(name: String, command: String, args: [String]) {
        let session = TUISession(name: name, command: command, arguments: args)
        sessions.append(session)
        
        let tabViewItem = NSTabViewItem(viewController: session.viewController)
        tabViewItem.label = name
        tabView.addTabViewItem(tabViewItem)
        
        session.start()
    }
}

/// Represents a single TUI session with its own PTY
class TUISession {
    let name: String
    let command: String
    let arguments: [String]
    let viewController: TerminalViewController
    private var process: Process?
    private var masterFD: Int32 = -1
    
    init(name: String, command: String, arguments: [String]) {
        self.name = name
        self.command = command
        self.arguments = arguments
        self.viewController = TerminalViewController()
    }
    
    func start() {
        // Create PTY pair
        var masterFD: Int32 = -1
        var slaveFD: Int32 = -1
        
        guard openpty(&masterFD, &slaveFD, nil, nil, nil) == 0 else {
            viewController.appendOutput("Failed to create PTY\n")
            return
        }
        
        self.masterFD = masterFD
        
        // Start reading from PTY
        startReadingOutput(from: masterFD)
        
        // Spawn process
        process = Process()
        process?.executableURL = URL(fileURLWithPath: command)
        process?.arguments = arguments
        
        // Redirect I/O to PTY slave
        let slaveFileHandle = FileHandle(fileDescriptor: slaveFD, closeOnDealloc: true)
        process?.standardInput = slaveFileHandle
        process?.standardOutput = slaveFileHandle
        process?.standardError = slaveFileHandle
        
        do {
            try process?.run()
        } catch {
            viewController.appendOutput("Failed to start process: \(error)\n")
        }
        
        close(slaveFD)
    }
    
    private func startReadingOutput(from fd: Int32) {
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            var buffer = [UInt8](repeating: 0, count: 8192)
            
            while true {
                let bytesRead = read(fd, &buffer, buffer.count)
                
                if bytesRead <= 0 {
                    break
                }
                
                if let output = String(bytes: buffer[..<bytesRead], encoding: .utf8) {
                    DispatchQueue.main.async {
                        self?.viewController.appendOutput(output)
                    }
                }
            }
        }
    }
    
    deinit {
        if masterFD >= 0 {
            close(masterFD)
        }
        process?.terminate()
    }
}

/// View controller for displaying terminal output
class TerminalViewController: NSViewController {
    var scrollView: NSScrollView!
    var textView: NSTextView!
    
    override func loadView() {
        view = NSView(frame: NSRect(x: 0, y: 0, width: 800, height: 600))
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        
        setupTextView()
    }
    
    private func setupTextView() {
        // Create scroll view
        scrollView = NSScrollView(frame: view.bounds)
        scrollView.autoresizingMask = [.width, .height]
        scrollView.hasVerticalScroller = true
        scrollView.hasHorizontalScroller = true
        
        // Create text view
        let textContainer = NSTextContainer()
        let layoutManager = NSLayoutManager()
        let textStorage = NSTextStorage()
        
        layoutManager.addTextContainer(textContainer)
        textStorage.addLayoutManager(layoutManager)
        
        textView = NSTextView(frame: scrollView.bounds, textContainer: textContainer)
        textView.autoresizingMask = [.width, .height]
        textView.isEditable = false
        textView.isSelectable = true
        textView.font = NSFont.monospacedSystemFont(ofSize: 12, weight: .regular)
        textView.textColor = .white
        textView.backgroundColor = .black
        
        scrollView.documentView = textView
        view.addSubview(scrollView)
    }
    
    func appendOutput(_ output: String) {
        textView.textStorage?.append(NSAttributedString(
            string: output,
            attributes: [
                .font: NSFont.monospacedSystemFont(ofSize: 12, weight: .regular),
                .foregroundColor: NSColor.white
            ]
        ))
        
        // Auto-scroll to bottom
        textView.scrollToEndOfDocument(nil)
    }
}
