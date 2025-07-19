# Pain Points: Using browser-cli as Claude

## Overview
As an AI assistant using browser-cli to explore web applications, I encountered several challenges that made complex tasks like authentication and dynamic content interaction difficult. This document outlines the pain points and suggests improvements to make browser-cli more powerful for automation tasks.

## 🎉 UPDATE: Tmux Changes Everything!

After struggling with the issues below, I discovered that using browser-cli with tmux solves the biggest pain point - session persistence! Here's what works:

### The Tmux Solution
```bash
# Start browser-cli console in tmux
tmux new-session -d -s browser-session "browser-cli console"

# Send commands to the persistent session
tmux send-keys -t browser-session "navigate https://localhost:3000" Enter
tmux send-keys -t browser-session "click 'Sign In'" Enter
tmux send-keys -t browser-session "fill '#email' 'user@example.com'" Enter
# ... browser stays alive between commands!
```

### What This Solved:
✅ **Session persistence** - Same browser instance across all commands  
✅ **Multi-step workflows** - Can complete complex authentication flows  
✅ **Navigation works** - Back/forward buttons maintain state  
✅ **OAuth redirects** - Browser follows redirects without losing context  
✅ **Form state** - Filled forms stay filled  

### Real Success Story:
```bash
# This actually worked with tmux!
tmux new-session -d -s browser-session "browser-cli console"
tmux send-keys -t browser-session "navigate https://localhost:3000" Enter
tmux send-keys -t browser-session "js Array.from(document.querySelectorAll('a')).find(a => a.textContent.includes('Sign In')).click()" Enter
# Successfully navigated to auth page in SAME browser!
tmux send-keys -t browser-session "js document.querySelector('input[type=\"password\"]').value = 'password'" Enter
tmux send-keys -t browser-session "js Array.from(document.querySelectorAll('button')).find(b => b.textContent.includes('Continue')).click()" Enter
# OAuth redirect worked! Browser maintained state through the entire flow
```

### Recommendation:
**Tmux integration should be the default mode** or at least heavily documented as the preferred way to use browser-cli for any serious automation work.

### Proposed Tmux Integration:
```bash
# Built-in session management
browser-cli session start my-app     # Creates tmux session automatically
browser-cli session list             # Shows active sessions
browser-cli session attach my-app    # Attaches to existing session

# Simplified commands that auto-route to tmux session
browser-cli --session my-app navigate https://example.com
browser-cli --session my-app click "#login"
browser-cli --session my-app fill "#email" "user@example.com"

# Or even simpler with environment variable
export BROWSER_SESSION=my-app
browser-cli navigate https://example.com  # Uses tmux session automatically
browser-cli click "#login"               # Same session!
```

## 🔴 Critical Pain Points (Without Tmux)

### 1. **No Session Persistence**
**Problem**: Every command creates a new browser instance, losing all state
```bash
browser-cli navigate https://example.com  # New browser
browser-cli click "#login"                # Different browser!
```

**Impact**: 
- Impossible to complete multi-step workflows
- Authentication flows broken
- Can't maintain logged-in state
- Shopping carts, forms, etc. all lost

**Solution Ideas**:
- `--session <name>` flag to reuse browser instances
- Integration with tmux for persistent sessions
- Background daemon mode that keeps browser alive

### 2. **Console Mode Limitations**
**Problem**: Console mode exits when piping commands
```bash
echo "navigate https://example.com" | browser-cli console  # Exits immediately
```

**Impact**:
- Had to create complex echo scripts
- No interactive exploration possible
- Can't react to page state

**Solution Ideas**:
- `--script <file>` mode that doesn't exit
- `--interactive` flag that stays open
- Better REPL that can handle both piped and interactive input

### 3. **Timing & Dynamic Content Issues**
**Problem**: No reliable way to wait for content
```javascript
// This pattern failed constantly:
js new Promise(r => setTimeout(r, 3000))  // Arbitrary waits
```

**Impact**:
- Race conditions with React/Vue apps
- Forms not ready when trying to fill
- Page navigations break execution context

**Solution Ideas**:
- `wait-for-selector <selector>` command
- `wait-for-text <text>` command
- `wait-until-stable` for dynamic content
- Built-in retries with exponential backoff

## 🟡 Major Usability Issues

### 4. **Poor Error Messages**
**Problem**: Cryptic errors that don't help debugging
```
Error: Error -32000: DOM Error while querying
Error: ExceptionDetails { exception_id: 1, text: "Uncaught", line_number: 0...
```

**Impact**:
- Don't know if selector is wrong or element doesn't exist
- Can't tell if page hasn't loaded or selector is invalid
- Stack traces are unhelpful

**Solution Ideas**:
- Human-friendly error messages
- Suggest similar selectors if not found
- Screenshot on error for debugging
- `--verbose` mode with detailed logs

### 5. **Limited Selector Support**
**Problem**: Only basic CSS selectors work
```bash
browser-cli click "button:has-text('Sign In')"  # Doesn't work
browser-cli click "text=Sign In"                # Would be nice
```

**Solution Ideas**:
- Support Playwright-style selectors
- XPath support
- Text-based selectors
- Accessibility selectors (aria-label, role)

### 6. **No Debugging Tools**
**Problem**: Can't see what's happening in the browser
- No access to console logs
- Can't see network requests
- No way to inspect elements
- Screenshots are only debugging tool

**Solution Ideas**:
- `console-logs` command to stream browser console
- `network` command to see requests
- `inspect <selector>` to get element details
- `--debug` flag to keep browser visible

## 🟢 Quality of Life Improvements

### 7. **Better Authentication Support**
**Problem**: Modern auth flows are complex
- OAuth redirects lose context
- 2FA is impossible
- Can't handle popups

**Solution Ideas**:
```bash
browser-cli auth login --email user@example.com --password xxx
browser-cli auth wait-for-redirect
browser-cli auth handle-oauth-popup
```

### 8. **Workflow Automation**
**Problem**: Complex tasks require too much manual scripting

**Solution Ideas**:
```yaml
# browser-workflow.yml
name: Login and Scrape
steps:
  - navigate: https://example.com
  - click: "#login"
  - fill:
      "#email": "user@example.com"
      "#password": "${PASSWORD}"
  - click: "button[type=submit]"
  - wait-for: "#dashboard"
  - extract:
      title: "h1"
      data: ".data-table"
```

### 9. **Session Management**
**Solution**: Tmux integration
```bash
# Start persistent session
browser-cli session start my-app

# Use in tmux
tmux new -s browser-session
browser-cli console --persistent

# Attach from anywhere
browser-cli session attach my-app
```

### 10. **State Inspection**
**Problem**: Can't query browser state effectively

**Solution Ideas**:
```bash
browser-cli state cookies
browser-cli state local-storage
browser-cli state session-storage
browser-cli state url
browser-cli state title
```

## 🚀 Dream Features

### Advanced Selectors
```bash
browser-cli click "text=Sign In"
browser-cli click "aria/Login Button"
browser-cli click "xpath=//button[contains(text(), 'Submit')]"
browser-cli click ".button:visible"
```

### Smart Waiting
```bash
browser-cli wait network-idle
browser-cli wait dom-stable
browser-cli wait animation-complete
browser-cli wait custom "document.readyState === 'complete'"
```

### Powerful Extraction
```bash
browser-cli extract --json "
{
  title: 'h1',
  price: '.price',
  description: 'p.description',
  images: ['img.product @src']
}
"
```

### Conditional Logic
```bash
browser-cli if-exists "#cookie-banner" click "#accept-cookies"
browser-cli while-exists ".load-more" click ".load-more"
```

### Recording & Playback
```bash
browser-cli record start
# Perform actions in browser
browser-cli record stop --save login-flow.json
browser-cli replay login-flow.json
```

## Implementation Priority (Updated After Tmux Discovery)

1. ✅ **Session persistence** - SOLVED with tmux! Document this as the primary usage pattern
2. **Built-in tmux wrapper** - Make `browser-cli --daemon` or `browser-cli session` commands that handle tmux automatically
3. **Better waiting mechanisms** (Still needed for reliability)
4. **Improved error messages** (Developer experience)
5. **Workflow files** (Automation at scale with tmux sessions)

## Real-World Example: Logging into Influenceable

Here's what I actually had to do to try logging in:

```bash
# Attempt 1: Naive approach (failed - different browser instances)
browser-cli navigate https://localhost:3000
browser-cli click "Sign In"  # New browser, lost context!

# Attempt 2: Console mode with echo (failed - timing issues)
echo -e "navigate https://localhost:3000\nclick 'Sign In'" | browser-cli console

# Attempt 3: Complex script (partially worked but lost context on redirect)
cat > /tmp/login.txt << 'EOF'
navigate https://localhost:3000/auth
js new Promise(r => setTimeout(r, 3000))
js document.querySelector('input[type="email"]').value = 'cole@influenceable.io'
js document.querySelector('input[type="password"]').value = 'password'
js Array.from(document.querySelectorAll('button')).find(b => b.textContent.includes('Continue')).click()
EOF
browser-cli console < /tmp/login.txt

# What I wished I could do:
browser-cli session start influenceable
browser-cli navigate https://localhost:3000
browser-cli click "text=Sign In"
browser-cli wait-for "input[type=email]"
browser-cli fill "#email" "cole@influenceable.io"
browser-cli fill "#password" "password"
browser-cli click "text=Continue"
browser-cli wait-for-navigation
browser-cli screenshot logged-in.png
```

The authentication flow was nearly impossible because:
1. Clerk auth redirected to a different domain
2. Lost all context between commands
3. No way to handle the OAuth flow
4. Timing was unpredictable with React hydration

## Conclusion

The browser-cli tool has great potential but currently struggles with modern web applications. The core issue is the lack of session persistence, which makes multi-step workflows nearly impossible. With these improvements, it could become an incredibly powerful tool for web automation, testing, and scraping.

The most impactful change would be implementing persistent sessions, possibly through a daemon mode or tmux integration. This alone would solve 50% of the current limitations and make the tool actually usable for real-world automation tasks.

As Claude, I want browser-cli to be my "hands" for web interaction. Right now, it's like having hands that forget what they just touched every time I use them. With session persistence and better waiting mechanisms, I could effectively browse, test, and automate web tasks just like a human would.