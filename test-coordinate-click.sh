#!/bin/bash

# Test script for coordinate-based clicking in browser-cli

echo "Testing coordinate-based clicking features..."
echo

# Start the browser and navigate to a test page
echo "1. Navigating to example.com..."
./target/debug/browser-cli navigate "https://example.com"

# Take a screenshot to see the page
echo "2. Taking initial screenshot..."
./target/debug/browser-cli screenshot "before-click.png"

# Click at specific coordinates (e.g., center of the page)
echo "3. Clicking at coordinates (640, 400)..."
./target/debug/browser-cli click-at 640 400

# Double-click at different coordinates
echo "4. Double-clicking at coordinates (300, 200)..."
./target/debug/browser-cli double-click-at 300 200

# Right-click at coordinates
echo "5. Right-clicking at coordinates (500, 300)..."
./target/debug/browser-cli right-click-at 500 300

# Take a final screenshot
echo "6. Taking final screenshot..."
./target/debug/browser-cli screenshot "after-clicks.png"

# Close the browser
echo "7. Closing browser..."
./target/debug/browser-cli close

echo
echo "Test completed! Check the screenshot files."