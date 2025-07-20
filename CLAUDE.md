# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository. You must practice test driven development, meaning you are able to build/test the tool and use it yourself at any point. Also Please use GH issue when problems come up. Create TODO list on the GH issues that you can use. Always remember you are building this tool for yourself. It will be used for dev and ui QA/testing.

## Project Overview

**browser-cli** is a high-performance command-line browser automation tool built with Rust and Chrome DevTools Protocol. It provides both direct CLI commands and an interactive console for controlling Chrome programmatically.

## Architecture

The codebase follows a modular architecture:

- **main.rs**: CLI entry point with command parsing and orchestration
- **browser.rs**: Core browser control logic using chromiumoxide crate
- **console.rs**: Interactive console with rustyline for REPL functionality

## Key Features

### Browser Control
- Chrome automation via chromiumoxide (CDP wrapper)
- Async/await based operations with tokio runtime
- Automatic browser lifecycle management with temp directories
- Signal handling for graceful shutdown

### Interaction Methods
- **Selector-based**: CSS selectors for element targeting
- **Coordinate-based**: Precise mouse interactions at (x,y) coordinates
- **JavaScript execution**: Direct script execution and result retrieval
- **Visual automation**: Screenshot-based workflow validation

### Session Management
- Persistent browser sessions via tmux integration
- State management across commands
- Automatic cleanup of temporary Chrome data directories

## Development Commands

### Build & Run
```bash
# Development build
cargo build

# Production build
cargo build --release

# Run with CLI arguments
cargo run -- navigate https://example.com
cargo run -- console

# Run tests (when implemented)
cargo test
```

### Common Usage Patterns

**Direct CLI usage:**
```bash
# Navigate and take screenshot
./target/debug/browser-cli navigate https://github.com
./target/debug/browser-cli screenshot github.png

# Coordinate-based interaction
./target/debug/browser-cli click-at 640 400
./target/debug/browser-cli double-click-at 300 200
```

**Interactive console:**
```bash
./target/debug/browser-cli console
browser> navigate https://example.com
browser> screenshot
browser> clickat 500 300
browser> js document.title
```

**Tmux integration (recommended):**
```bash
# Create persistent browser session
tmux new-session -d -s browser "./target/debug/browser-cli console"

# Send commands to session
tmux send-keys -t browser "navigate https://github.com" Enter
tmux send-keys -t browser "screenshot github.png" Enter
```

## Coordinate-Based Automation

The tool implements precise coordinate-based interactions:

- `click-at x y` - Single left-click
- `double-click-at x y` - Double left-click  
- `right-click-at x y` - Right-click context menu

**Implementation details:**
- Uses Chrome DevTools `Input.dispatchMouseEvent` protocol
- Three-step process: move → press → release
- Supports click counts for double-clicking
- Works with any screen resolution (coordinates are absolute)

## Testing

**Visual automation test:**
```bash
./test-coordinate-click.sh
```

This script demonstrates:
1. Navigation to test page
2. Screenshot capture
3. Coordinate-based clicks (single, double, right)
4. Cleanup with browser closure

## Dependencies

**Core stack:**
- `chromiumoxide` - Chrome automation via CDP
- `tokio` - Async runtime
- `clap` - CLI argument parsing
- `rustyline` - Interactive console REPL
- `colored` - Terminal output styling

## Code Structure

```
src/
├── main.rs      # CLI entry point, command routing, signal handling
├── browser.rs   # Core browser operations, CDP interactions
└── console.rs   # Interactive console, command parsing, REPL
```

**BrowserController (browser.rs)**:
- Manages browser lifecycle (launch, close, cleanup)
- Implements all interaction methods (navigate, click, type, etc.)
- Handles coordinate-based mouse operations
- Provides JavaScript execution capabilities

**Console (console.rs)**:
- Interactive command-line interface
- Command history and tab completion
- Rich help system with categorized commands
- Real-time browser state management

## Visual Automation Workflow

1. **Screenshot analysis**: Use `screenshot` to capture page state
2. **Coordinate identification**: Visually identify target coordinates
3. **Execute interaction**: Use coordinate-based commands
4. **Verify results**: Take follow-up screenshots or check page state

This approach is particularly useful when:
- CSS selectors are unreliable or unavailable
- Working with canvas/SVG elements
- Testing responsive designs across screen sizes
- Automating legacy web applications

## Error Handling

- Graceful browser cleanup on Ctrl+C
- Temporary directory cleanup on close
- Informative error messages for common failure modes
- Fallback mechanisms for element detection
