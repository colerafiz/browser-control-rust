# Visual Automation with Browser CLI

## Overview

Browser CLI supports advanced visual automation capabilities that go beyond traditional CSS selector-based interactions. This enables more robust and intuitive browser automation by leveraging visual coordinates and screenshot analysis.

## Coordinate-Based Clicking

The browser CLI implements precise coordinate-based mouse interactions through the Chrome DevTools Protocol (CDP).

### Available Methods

#### 1. Click at Coordinates
```bash
# CLI usage
browser-cli click-at 640 400

# Interactive console
clickat 640 400
```

#### 2. Double-Click at Coordinates
```bash
# CLI usage  
browser-cli double-click-at 300 200

# Interactive console
doubleclickat 300 200
```

#### 3. Right-Click at Coordinates
```bash
# CLI usage
browser-cli right-click-at 500 300

# Interactive console
rightclickat 500 300
```

## Implementation Details

### Chrome DevTools Protocol Integration

The coordinate-based clicking uses Chrome's `Input.dispatchMouseEvent` command with the following sequence:

1. **Mouse Move**: Position cursor at target coordinates
2. **Mouse Down**: Press the specified mouse button 
3. **Mouse Up**: Release the mouse button

### Technical Implementation

```rust
pub async fn click_at_coordinates(&self, x: f64, y: f64) -> Result<()> {
    // Move mouse to coordinates
    let move_cmd = DispatchMouseEventParams::builder()
        .x(x)
        .y(y)
        .r#type(DispatchMouseEventType::MouseMoved)
        .build()?;
    page.execute(move_cmd).await?;
    
    // Mouse down
    let down_cmd = DispatchMouseEventParams::builder()
        .x(x)
        .y(y)
        .button(MouseButton::Left)
        .r#type(DispatchMouseEventType::MousePressed)
        .click_count(1)
        .build()?;
    page.execute(down_cmd).await?;
    
    // Mouse up
    let up_cmd = DispatchMouseEventParams::builder()
        .x(x)
        .y(y) 
        .button(MouseButton::Left)
        .r#type(DispatchMouseEventType::MouseReleased)
        .click_count(1)
        .build()?;
    page.execute(up_cmd).await?;
}
```

## JavaScript Element Detection

An alternative approach uses JavaScript's `document.elementFromPoint()` API:

```javascript
// Get element at coordinates and click it
js document.elementFromPoint(642, 41).click()
```

This method:
- Finds the topmost element at the specified coordinates
- Triggers a programmatic click event
- Works even when elements don't have convenient CSS selectors

## Visual Automation Workflow

1. **Take Screenshot**: Capture current page state
   ```bash
   screenshot before-interaction.png
   ```

2. **Analyze Visually**: Identify target element coordinates from screenshot

3. **Execute Click**: Use coordinate-based clicking
   ```bash
   clickat 642 41
   ```

4. **Verify Result**: Take another screenshot or check page state
   ```bash
   screenshot after-interaction.png
   ```

## Advantages of Visual Automation

### Robustness
- Works when CSS selectors are unreliable or non-existent
- Handles dynamic content and shadow DOM elements
- Functions with complex JavaScript frameworks

### Simplicity
- No need to inspect DOM structure
- Visual identification of click targets
- Direct translation from manual testing to automation

### Flexibility
- Works across different websites and applications
- Adapts to UI changes that break selector-based automation
- Enables interaction with canvas, SVG, and other non-standard elements

## Real-World Example

During testing with the Influenceable app, traditional selector-based clicking failed to locate the "Sign In" button reliably. Using visual automation:

1. Took screenshot of the page
2. Visually identified "Sign In" button at coordinates (642, 41)
3. Successfully clicked using `clickat 642 41`
4. Navigation to authentication page confirmed success

## Best Practices

### Coordinate Accuracy
- Use screenshots to verify exact coordinates
- Account for browser window size and zoom level
- Test coordinates across different screen resolutions

### Error Handling
- Take screenshots before and after interactions
- Verify page state changes after clicks
- Use JavaScript element detection as fallback

### Session Management
- Use tmux for persistent browser sessions
- Maintain browser state across multiple interactions
- Handle navigation and redirects gracefully

## Integration with Other Tools

The visual automation capabilities complement other browser CLI features:

- **Screenshots**: Document automation steps and verify results
- **JavaScript Execution**: Enhanced element detection and interaction
- **Text Extraction**: Verify content changes after interactions
- **Navigation**: Handle redirects and page changes

## Future Enhancements

Potential improvements to visual automation:

1. **Image Recognition**: Automatic element detection from screenshots
2. **Relative Positioning**: Click relative to detected elements
3. **Drag and Drop**: Coordinate-based drag operations
4. **Viewport Adaptation**: Automatic coordinate scaling
5. **Visual Assertions**: Screenshot-based testing validation