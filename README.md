# Browser Control CLI (Rust Version)

A high-performance command line tool for browser automation using `chromiumoxide`. Control a Chrome browser programmatically from the command line with Rust's speed and safety.

## Installation

```bash
cargo build --release
# Optional: Copy to PATH
cp target/release/browser-cli /usr/local/bin/
```

## Usage

### Basic Commands

Navigate to a page:
```bash
cargo run navigate https://example.com
# or using alias
cargo run go https://example.com
```

Take a screenshot:
```bash
cargo run screenshot
cargo run screenshot my-screenshot.png
```

Click an element:
```bash
cargo run click "button.submit"
cargo run click "#login-button"
```

Type text into an input:
```bash
cargo run type "#search" "hello world"
cargo run type "input[name='username']" "myusername"
```

Scroll the page:
```bash
cargo run scroll down
cargo run scroll up 500
cargo run scroll top
cargo run scroll bottom
```

Search on the current page:
```bash
cargo run search "my search query"
```

Get text content:
```bash
cargo run text                    # Get page title and URL
cargo run text "h1"              # Get text from first h1
cargo run text ".article-title"  # Get text from element
```

Close the browser:
```bash
cargo run close
```

### Coordinate-Based Clicking

For precise interactions when CSS selectors aren't reliable:

```bash
# Click at specific coordinates
cargo run click-at 640 400

# Double-click at coordinates  
cargo run double-click-at 300 200

# Right-click at coordinates
cargo run right-click-at 500 300
```

### Interactive Console Mode

Launch an interactive REPL for complex automation workflows:

```bash
cargo run console
```

In console mode, you can use shortened commands:
```
browser> go https://example.com
browser> ss screenshot.png
browser> clickat 642 41
browser> js document.elementFromPoint(642, 41).click()
browser> exit
```

### Example Workflow

```bash
# Navigate to a website
cargo run navigate https://github.com

# Take a screenshot
cargo run screenshot github-home.png

# Search for something
cargo run search "browser automation"

# Scroll down to see more results
cargo run scroll down

# Click on a result
cargo run click "a[data-testid='search-result']"

# Close when done
cargo run close
```

## Features

- **High Performance**: Built with Rust for speed and memory efficiency
- **Async/Await**: Non-blocking operations using Tokio
- **Browser Control**: Launch and control Chrome browser instances
- **Element Interaction**: Click, type, and interact with page elements
- **Visual Automation**: Coordinate-based clicking for precise interactions
- **Interactive Console**: REPL mode for complex automation workflows
- **JavaScript Execution**: Run custom JavaScript code in the browser
- **Scrolling**: Scroll in any direction with custom amounts
- **Screenshots**: Capture full-page screenshots
- **Text Extraction**: Get text content from elements or page info
- **Search**: Automatically find and use search inputs on pages
- **Session Persistence**: Works with tmux for maintaining browser state
- **Graceful Shutdown**: Properly close browser on exit signals

## For Claude Code Integration

This tool is designed to work with Claude Code for web automation tasks. After building, Claude can use commands like:

```bash
./target/release/browser-cli navigate https://example.com
./target/release/browser-cli click "#menu-button"
./target/release/browser-cli type "#search-input" "search query"
./target/release/browser-cli screenshot current-page.png
./target/release/browser-cli scroll down 300
./target/release/browser-cli text ".main-content"
./target/release/browser-cli click-at 642 41  # Visual automation
./target/release/browser-cli console          # Interactive mode
./target/release/browser-cli close
```

### Visual Automation Example

When traditional selectors fail, use coordinate-based clicking:

```bash
# Take a screenshot to identify target coordinates
./target/release/browser-cli screenshot page.png

# Click at specific coordinates (e.g., a button at x=642, y=41)
./target/release/browser-cli click-at 642 41

# Alternative: Use JavaScript element detection
./target/release/browser-cli console
browser> js document.elementFromPoint(642, 41).click()
```

See [visual-automation.md](visual-automation.md) for detailed documentation.

## Dependencies

- `chromiumoxide` - Chrome DevTools Protocol implementation
- `clap` - Command line argument parser
- `tokio` - Async runtime
- `anyhow` - Error handling
- `colored` - Terminal colors
- `chrono` - Date/time utilities

## Advantages over TypeScript Version

- **Performance**: Significantly faster startup and execution
- **Memory Usage**: Lower memory footprint
- **Type Safety**: Compile-time guarantees
- **Single Binary**: No need for Node.js runtime
- **Concurrency**: Better handling of async operations