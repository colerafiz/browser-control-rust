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
- **Scrolling**: Scroll in any direction with custom amounts
- **Screenshots**: Capture full-page screenshots
- **Text Extraction**: Get text content from elements or page info
- **Search**: Automatically find and use search inputs on pages
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
./target/release/browser-cli close
```

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