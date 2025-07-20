use anyhow::Result;
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotParams;
use chromiumoxide::cdp::browser_protocol::input::{DispatchMouseEventParams, DispatchMouseEventType, MouseButton};
use chromiumoxide::{Browser, BrowserConfig, Page};
use colored::*;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::fs;
use chrono::{DateTime, Utc};
use thirtyfour::prelude::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

pub struct BrowserController {
    browser: Option<Browser>,
    page: Option<Page>,
    temp_dir: Option<String>,
}

impl BrowserController {
    pub fn new() -> Self {
        Self {
            browser: None,
            page: None,
            temp_dir: None,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        if self.browser.is_some() {
            return Ok(());
        }

        // Create a temporary user data directory to avoid conflicts with existing Chrome sessions
        let temp_dir = format!("/tmp/browser-cli-{}-{}", std::process::id(), chrono::Utc::now().timestamp());
        
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .window_size(1280, 800)
                .user_data_dir(&temp_dir)
                .build()
                .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to launch browser. Make sure Chrome is installed. Error: {}", e))?;

        let _handle = tokio::task::spawn(async move {
            while let Some(h) = handler.next().await {
                if let Err(_) = h {
                    // Suppress handler errors
                }
            }
        });

        let page = browser.new_page("about:blank").await?;
        
        self.browser = Some(browser);
        self.page = Some(page);
        self.temp_dir = Some(temp_dir);
        
        println!("{} Browser ready", "🚀".green());
        Ok(())
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        self.ensure_initialized().await?;
        
        println!("{}", format!("Navigating to: {}", url).blue());
        
        let page = self.page.as_ref().unwrap();
        page.goto(url).await?;
        
        // Wait for navigation to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Get concise page information for AI/agents
        let page_info = self.get_concise_page_info().await?;
        println!("{} {}", "✓".green(), page_info);
        
        Ok(())
    }

    pub async fn screenshot(&self, filename: Option<&str>) -> Result<String> {
        self.ensure_page()?;
        
        // Create browser-ss directory if it doesn't exist
        let screenshots_dir = "browser-ss";
        if let Err(_) = fs::metadata(screenshots_dir) {
            fs::create_dir_all(screenshots_dir)?;
        }
        
        let final_filename = if let Some(name) = filename {
            // If user provides filename, use it directly
            if name.starts_with('/') || name.contains('/') {
                name.to_string()
            } else {
                format!("{}/{}", screenshots_dir, name)
            }
        } else {
            // Generate filename based on route and timestamp
            let page = self.page.as_ref().unwrap();
            let url = page.url().await?.unwrap_or_default();
            let route = self.url_to_route(&url);
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            format!("{}/{}_{}.png", screenshots_dir, route, timestamp)
        };
        
        let path = PathBuf::from(&final_filename);
        
        let page = self.page.as_ref().unwrap();
        let screenshot = page.screenshot(CaptureScreenshotParams::builder().build()).await?;
        tokio::fs::write(&path, screenshot).await?;
        
        println!("{} Screenshot: {}", "📸".cyan(), final_filename);
        Ok(final_filename)
    }

    pub async fn click(&self, selector: &str) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let element = page.find_element(selector).await?;
        element.click().await?;
        
        println!("{} Clicked: {}", "✓".green(), selector);
        Ok(())
    }

    pub async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let element = page.find_element(selector).await?;
        element.click().await?;
        element.type_str(text).await?;
        
        println!("{} Typed into {}", "✓".green(), selector);
        Ok(())
    }

    pub async fn scroll(&self, direction: &str, amount: Option<i32>) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        match direction {
            "up" => {
                let scroll_amount = -(amount.unwrap_or(300));
                page.evaluate(format!("window.scrollBy(0, {})", scroll_amount)).await?;
            }
            "down" => {
                let scroll_amount = amount.unwrap_or(300);
                page.evaluate(format!("window.scrollBy(0, {})", scroll_amount)).await?;
            }
            "top" => {
                page.evaluate("window.scrollTo(0, 0)").await?;
            }
            "bottom" => {
                page.evaluate("window.scrollTo(0, document.body.scrollHeight)").await?;
            }
            _ => return Err(anyhow::anyhow!("Invalid scroll direction")),
        }
        
        println!("{} Scrolled {}", "✓".green(), direction);
        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Searching for: '{}'", query).blue());
        
        let page = self.page.as_ref().unwrap();
        
        let search_selectors = vec![
            "input[type=\"search\"]",
            "input[placeholder*=\"search\" i]",
            "input[name*=\"search\" i]",
            "input[id*=\"search\" i]",
            ".search input",
            "#search input",
        ];
        
        for selector in search_selectors {
            if let Ok(element) = page.find_element(selector).await {
                element.click().await?;
                element.type_str(query).await?;
                page.evaluate("document.activeElement.dispatchEvent(new KeyboardEvent('keydown', {key: 'Enter', code: 'Enter'}))").await?;
                println!("{} Search: {}", "✓".green(), query);
                return Ok(());
            }
        }
        
        Err(anyhow::anyhow!("No search input found on page"))
    }

    pub async fn get_text(&self, selector: Option<&str>) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        if let Some(sel) = selector {
            println!("{}", format!("Getting text from: {}", sel).blue());
            let element = page.find_element(sel).await?;
            let text = element.inner_text().await?;
            Ok(text.unwrap_or_default())
        } else {
            println!("{}", "Getting page title and URL".blue());
            let title = page.get_title().await?.unwrap_or_default();
            let url = page.url().await?;
            Ok(format!("Title: {}\nURL: {}", title, url.unwrap_or_default()))
        }
    }

    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut browser) = self.browser.take() {
            println!("{}", "Closing browser...".yellow());
            browser.close().await?;
            self.page = None;
            
            // Clean up temporary directory
            if let Some(temp_dir) = &self.temp_dir {
                if let Err(e) = std::fs::remove_dir_all(temp_dir) {
                    eprintln!("Warning: Failed to remove temp directory {}: {}", temp_dir, e);
                }
            }
            self.temp_dir = None;
            
            println!("{}", "Browser closed".green());
        }
        Ok(())
    }

    async fn ensure_initialized(&mut self) -> Result<()> {
        if self.browser.is_none() {
            self.init().await?;
        }
        Ok(())
    }

    fn ensure_page(&self) -> Result<()> {
        if self.page.is_none() {
            return Err(anyhow::anyhow!("Browser not initialized"));
        }
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.browser.is_some() && self.page.is_some()
    }

    pub async fn execute_javascript(&self, code: &str) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let result = page.evaluate(code).await?;
        
        if let Some(value) = result.value() {
            println!("{}", serde_json::to_string_pretty(value)?);
        }
        
        Ok(())
    }

    pub async fn get_url(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let url = page.url().await?;
        Ok(url.unwrap_or_default())
    }

    pub async fn get_title(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let title = page.get_title().await?;
        Ok(title.unwrap_or_default())
    }

    pub async fn reload(&self) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", "Reloading page...".blue());
        
        let page = self.page.as_ref().unwrap();
        page.reload().await?;
        
        println!("{}", "Page reloaded".green());
        Ok(())
    }

    pub async fn go_back(&self) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", "Going back...".blue());
        
        let page = self.page.as_ref().unwrap();
        page.evaluate("window.history.back()").await?;
        
        println!("{}", "Navigated back".green());
        Ok(())
    }

    pub async fn go_forward(&self) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", "Going forward...".blue());
        
        let page = self.page.as_ref().unwrap();
        page.evaluate("window.history.forward()").await?;
        
        println!("{}", "Navigated forward".green());
        Ok(())
    }

    pub async fn click_at_coordinates(&self, x: f64, y: f64) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        // Perform click sequence
        let move_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .r#type(DispatchMouseEventType::MouseMoved)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse move command: {}", e))?;
        page.execute(move_cmd).await?;
        
        let down_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .r#type(DispatchMouseEventType::MousePressed)
            .click_count(1)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse down command: {}", e))?;
        page.execute(down_cmd).await?;
        
        let up_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .r#type(DispatchMouseEventType::MouseReleased)
            .click_count(1)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse up command: {}", e))?;
        page.execute(up_cmd).await?;
        
        println!("{} Clicked: ({}, {})", "✓".green(), x, y);
        Ok(())
    }

    pub async fn double_click_at_coordinates(&self, x: f64, y: f64) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Double-clicking at coordinates: ({}, {})", x, y).blue());
        
        let page = self.page.as_ref().unwrap();
        
        // Move mouse to coordinates
        let move_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .r#type(DispatchMouseEventType::MouseMoved)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse move command: {}", e))?;
        
        page.execute(move_cmd).await?;
        
        // Double click (mouse down with click_count=2)
        let down_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .r#type(DispatchMouseEventType::MousePressed)
            .click_count(2)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse down command: {}", e))?;
        
        page.execute(down_cmd).await?;
        
        // Mouse up with click_count=2
        let up_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Left)
            .r#type(DispatchMouseEventType::MouseReleased)
            .click_count(2)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse up command: {}", e))?;
        
        page.execute(up_cmd).await?;
        
        println!("{}", format!("Double-clicked at ({}, {})", x, y).green());
        Ok(())
    }

    pub async fn right_click_at_coordinates(&self, x: f64, y: f64) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Right-clicking at coordinates: ({}, {})", x, y).blue());
        
        let page = self.page.as_ref().unwrap();
        
        // Move mouse to coordinates
        let move_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .r#type(DispatchMouseEventType::MouseMoved)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse move command: {}", e))?;
        
        page.execute(move_cmd).await?;
        
        // Right click (mouse down)
        let down_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Right)
            .r#type(DispatchMouseEventType::MousePressed)
            .click_count(1)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse down command: {}", e))?;
        
        page.execute(down_cmd).await?;
        
        // Mouse up
        let up_cmd = DispatchMouseEventParams::builder()
            .x(x)
            .y(y)
            .button(MouseButton::Right)
            .r#type(DispatchMouseEventType::MouseReleased)
            .click_count(1)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build mouse up command: {}", e))?;
        
        page.execute(up_cmd).await?;
        
        println!("{}", format!("Right-clicked at ({}, {})", x, y).green());
        Ok(())
    }

    pub async fn wait_for_selector(&self, selector: &str, timeout_secs: Option<u64>) -> Result<()> {
        self.ensure_page()?;
        
        let timeout = timeout_secs.unwrap_or(10);
        println!("{}", format!("Waiting for selector '{}' (timeout: {}s)", selector, timeout).blue());
        
        let page = self.page.as_ref().unwrap();
        let start = std::time::Instant::now();
        
        while start.elapsed().as_secs() < timeout {
            if let Ok(element) = page.find_element(selector).await {
                println!("{}", format!("Element '{}' found", selector).green());
                return Ok(());
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Err(anyhow::anyhow!("Timeout waiting for selector: '{}' after {} seconds", selector, timeout))
    }

    pub async fn wait_for_text(&self, text: &str, timeout_secs: Option<u64>) -> Result<()> {
        self.ensure_page()?;
        
        let timeout = timeout_secs.unwrap_or(10);
        println!("{}", format!("Waiting for text '{}' (timeout: {}s)", text, timeout).blue());
        
        let page = self.page.as_ref().unwrap();
        let start = std::time::Instant::now();
        
        while start.elapsed().as_secs() < timeout {
            let body_text = page.evaluate("document.body.innerText").await?;
            if let Some(body_content) = body_text.value() {
                let content_str = body_content.to_string();
                if content_str.contains(text) {
                    println!("{}", format!("Text '{}' found", text).green());
                    return Ok(());
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Err(anyhow::anyhow!("Timeout waiting for text: '{}' after {} seconds", text, timeout))
    }

    pub async fn wait_for_navigation(&self, timeout_secs: Option<u64>) -> Result<()> {
        self.ensure_page()?;
        
        let timeout = timeout_secs.unwrap_or(30);
        println!("{}", format!("Waiting for navigation to complete (timeout: {}s)", timeout).blue());
        
        let page = self.page.as_ref().unwrap();
        let start = std::time::Instant::now();
        
        while start.elapsed().as_secs() < timeout {
            let ready_state = page.evaluate("document.readyState").await?;
            if let Some(state) = ready_state.value() {
                if state == "complete" {
                    println!("{}", "Navigation completed".green());
                    return Ok(());
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        
        Err(anyhow::anyhow!("Timeout waiting for navigation after {} seconds", timeout))
    }

    pub async fn highlight_element(&self, selector: &str) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Highlighting element: {}", selector).blue());
        
        let page = self.page.as_ref().unwrap();
        let element = page.find_element(selector).await?;
        
        // Add temporary highlight border
        let highlight_script = format!(
            r#"
            (function() {{
                const element = document.querySelector('{}');
                if (element) {{
                    element.style.border = '3px solid red';
                    element.style.outline = '2px solid yellow';
                    setTimeout(() => {{
                        element.style.border = '';
                        element.style.outline = '';
                    }}, 3000);
                    return true;
                }}
                return false;
            }})()
            "#,
            selector
        );
        
        let result = page.evaluate(highlight_script).await?;
        if let Some(found) = result.value() {
            if found.as_bool().unwrap_or(false) {
                println!("{}", format!("Highlighted element: {}", selector).green());
            } else {
                return Err(anyhow::anyhow!("Element not found: {}", selector));
            }
        }
        
        Ok(())
    }

    pub async fn get_cookies(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let cookies = page.get_cookies().await?;
        
        let cookie_json = serde_json::to_string_pretty(&cookies)?;
        Ok(cookie_json)
    }

    pub async fn get_local_storage(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let local_storage = page.evaluate("JSON.stringify(Object.entries(localStorage))").await?;
        
        if let Some(storage_data) = local_storage.value() {
            Ok(storage_data.to_string())
        } else {
            Ok("{}".to_string())
        }
    }

    pub async fn get_session_storage(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let session_storage = page.evaluate("JSON.stringify(Object.entries(sessionStorage))").await?;
        
        if let Some(storage_data) = session_storage.value() {
            Ok(storage_data.to_string())
        } else {
            Ok("{}".to_string())
        }
    }

    pub async fn clear_cookies(&self) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", "Clearing all cookies...".blue());
        
        let page = self.page.as_ref().unwrap();
        page.evaluate("document.cookie.split(';').forEach(cookie => { document.cookie = cookie.replace(/^ +/, '').replace(/=.*/, '=;expires=' + new Date().toUTCString() + ';path=/'); });").await?;
        
        println!("{}", "Cookies cleared".green());
        Ok(())
    }

    pub async fn set_cookie(&self, name: &str, value: &str, domain: Option<&str>) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let current_url = page.url().await?;
        let default_domain = "".to_string();
        let current_domain = current_url.as_ref().unwrap_or(&default_domain);
        
        let domain_str = domain.unwrap_or(current_domain);
        
        println!("{}", format!("Setting cookie: {}={} for domain: {}", name, value, domain_str).blue());
        
        page.evaluate(format!(
            "document.cookie = '{}={};domain={};path=/;'",
            name, value, domain_str
        )).await?;
        
        println!("{}", format!("Cookie set: {}={}", name, value).green());
        Ok(())
    }

    // Get concise page information for AI/agents
    pub async fn get_concise_page_info(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        // Get essential info only
        let title = page.get_title().await?.unwrap_or("Unknown".to_string());
        let url = page.url().await?.unwrap_or("Unknown".to_string());
        
        // Count key interactive elements only
        let element_counts = page.evaluate(
            r#"
            JSON.stringify({
                inputs: document.querySelectorAll('input:not([type="hidden"]), textarea, select').length,
                buttons: document.querySelectorAll('button, input[type="submit"], input[type="button"]').length,
                links: document.querySelectorAll('a[href]').length
            })
            "#
        ).await?;
        
        let mut info = format!("{} | {}", 
            title.chars().take(40).collect::<String>(),
            url.replace("https://", "").replace("http://", "")
        );
        
        if let Some(counts) = element_counts.value() {
            if let Ok(parsed) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(counts.clone()) {
                let inputs = parsed.get("inputs").and_then(|v| v.as_u64()).unwrap_or(0);
                let buttons = parsed.get("buttons").and_then(|v| v.as_u64()).unwrap_or(0);
                let links = parsed.get("links").and_then(|v| v.as_u64()).unwrap_or(0);
                
                if inputs > 0 || buttons > 0 || links > 0 {
                    info.push_str(&format!(" | i:{} b:{} l:{}", inputs, buttons, links));
                }
            }
        }
        
        Ok(info)
    }

    // Helper function to convert URL to route for screenshot naming
    fn url_to_route(&self, url: &str) -> String {
        if url.is_empty() || url == "about:blank" {
            return "blank".to_string();
        }
        
        let route = if let Ok(parsed_url) = url::Url::parse(url) {
            let host = parsed_url.host_str().unwrap_or("unknown");
            let path = parsed_url.path();
            
            // Clean up domain (remove www., special chars)
            let clean_host = host.replace("www.", "").replace(".", "_");
            
            // Clean up path (remove slashes, special chars, limit length)
            let clean_path = if path == "/" {
                "home".to_string()
            } else {
                path.replace("/", "_")
                    .replace("?", "_q_")
                    .replace("&", "_and_")
                    .replace("=", "_eq_")
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                    .take(30)
                    .collect()
            };
            
            format!("{}_{}", clean_host, clean_path)
        } else {
            // Fallback for invalid URLs
            url.chars()
                .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .take(20)
                .collect()
        };
        
        // Ensure we have a valid filename
        if route.is_empty() {
            "unknown".to_string()
        } else {
            route
        }
    }

    // Get concise status for AI/agents
    pub async fn get_status(&self) -> Result<String> {
        if !self.is_initialized() {
            return Ok("Browser not ready".to_string());
        }
        
        let page_info = self.get_concise_page_info().await?;
        Ok(page_info)
    }

    // Get key interactive elements for AI/agents (concise)
    pub async fn get_interactive_elements(&self) -> Result<String> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        let elements_info = page.evaluate(
            r#"
            JSON.stringify({
                inputs: Array.from(document.querySelectorAll('input:not([type="hidden"]), select, textarea')).filter(el => el.offsetParent !== null).map(el => ({
                    type: el.type || el.tagName.toLowerCase(),
                    id: el.id,
                    name: el.name,
                    placeholder: el.placeholder
                })).slice(0, 10),
                buttons: Array.from(document.querySelectorAll('button, input[type="submit"], input[type="button"]')).filter(el => el.offsetParent !== null).map(el => ({
                    text: (el.textContent || el.value || '').trim().substring(0, 30),
                    id: el.id
                })).slice(0, 8),
                links: Array.from(document.querySelectorAll('a[href]')).filter(el => el.offsetParent !== null && el.textContent.trim()).map(el => ({
                    text: el.textContent.trim().substring(0, 30),
                    href: el.href.substring(0, 50)
                })).slice(0, 8)
            })
            "#
        ).await?;
        
        if let Some(elements) = elements_info.value() {
            Ok(serde_json::to_string_pretty(elements)?)
        } else {
            Ok("No elements found".to_string())
        }
    }

    // Robust form filling method for tricky forms
    pub async fn fill_form_field(&self, selector: &str, value: &str) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        // Multi-step approach to ensure form field is properly filled
        let fill_script = format!(
            r#"
            (function() {{
                const element = document.querySelector('{}');
                if (!element) return false;
                
                // Focus the element first
                element.focus();
                
                // Clear existing value
                element.value = '';
                
                // Set the new value
                element.value = '{}';
                
                // Trigger multiple events to ensure form validation
                element.dispatchEvent(new Event('focus', {{bubbles: true}}));
                element.dispatchEvent(new Event('input', {{bubbles: true}}));
                element.dispatchEvent(new Event('change', {{bubbles: true}}));
                element.dispatchEvent(new Event('blur', {{bubbles: true}}));
                
                // Also try setting the value property again to be extra sure
                element.setAttribute('value', '{}');
                
                return element.value === '{}';
            }})()
            "#,
            selector, value, value, value
        );
        
        let result = page.evaluate(fill_script).await?;
        
        if let Some(success) = result.value() {
            if success.as_bool().unwrap_or(false) {
                println!("✓ Filled: {} = {}", selector, value);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to fill field: {}", selector))
            }
        } else {
            Err(anyhow::anyhow!("Field not found: {}", selector))
        }
    }

    // Submit form with validation bypass if needed
    pub async fn submit_form(&self, form_selector: Option<&str>) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        
        let submit_script = if let Some(selector) = form_selector {
            format!(
                r#"
                (function() {{
                    const form = document.querySelector('{}');
                    if (form) {{
                        form.submit();
                        return true;
                    }}
                    return false;
                }})()
                "#,
                selector
            )
        } else {
            r#"
            (function() {
                const form = document.querySelector('form');
                if (form) {
                    form.submit();
                    return true;
                }
                return false;
            })()
            "#.to_string()
        };
        
        let result = page.evaluate(submit_script).await?;
        
        if let Some(success) = result.value() {
            if success.as_bool().unwrap_or(false) {
                println!("✓ Form submitted");
                Ok(())
            } else {
                Err(anyhow::anyhow!("Form not found or submission failed"))
            }
        } else {
            Err(anyhow::anyhow!("Form submission failed"))
        }
    }

    // Ticker functionality for monitoring page changes
    pub async fn start_ticker(&self, selector: Option<&str>, interval_secs: u64, max_iterations: Option<u64>) -> Result<()> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let mut previous_state = HashMap::new();
        let mut iteration = 0;
        
        println!("{} Starting ticker ({}s intervals)...", "⏱️".cyan(), interval_secs);
        
        // Determine what to monitor
        let monitor_script = if let Some(sel) = selector {
            format!(
                r#"
                JSON.stringify({{
                    selector: '{}',
                    count: document.querySelectorAll('{}').length,
                    text: Array.from(document.querySelectorAll('{}')).map(el => el.textContent.trim()).join(' | '),
                    timestamp: Date.now()
                }})
                "#,
                sel, sel, sel
            )
        } else {
            r#"
            JSON.stringify({
                url: window.location.href,
                title: document.title,
                inputs: document.querySelectorAll('input:not([type="hidden"]), textarea').length,
                buttons: document.querySelectorAll('button, input[type="submit"], input[type="button"]').length,
                forms: document.querySelectorAll('form').length,
                timestamp: Date.now()
            })
            "#.to_string()
        };
        
        loop {
            // Check if we should stop
            if let Some(max) = max_iterations {
                if iteration >= max {
                    println!("{} Ticker completed {} iterations", "✓".green(), iteration);
                    break;
                }
            }
            
            // Get current state
            match page.evaluate(monitor_script.clone()).await {
                Ok(result) => {
                    if let Some(state_json) = result.value() {
                        if let Ok(state_str) = serde_json::to_string(state_json) {
                            let current_hash = format!("{:x}", md5::compute(&state_str));
                            
                            if let Some(prev_hash) = previous_state.get("hash") {
                                if prev_hash != &current_hash {
                                    println!("{} {} Change detected!", 
                                        "🔄".yellow(), 
                                        chrono::Utc::now().format("%H:%M:%S")
                                    );
                                    
                                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&state_str) {
                                        println!("  {}", parsed.to_string().dimmed());
                                    }
                                    
                                    previous_state.insert("hash".to_string(), current_hash);
                                } else {
                                    print!(".");
                                    std::io::Write::flush(&mut std::io::stdout()).ok();
                                }
                            } else {
                                // First iteration
                                println!("{} Baseline established", "📊".cyan());
                                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&state_str) {
                                    println!("  {}", parsed.to_string().dimmed());
                                }
                                previous_state.insert("hash".to_string(), current_hash);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{} Ticker error: {}", "⚠️".yellow(), e);
                }
            }
            
            iteration += 1;
            sleep(Duration::from_secs(interval_secs)).await;
        }
        
        Ok(())
    }

    // Enhanced wait-for with thirtyfour integration for better reliability
    pub async fn wait_for_element_enhanced(&self, selector: &str, timeout_secs: u64) -> Result<bool> {
        self.ensure_page()?;
        
        let page = self.page.as_ref().unwrap();
        let start_time = std::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);
        
        println!("{} Waiting for element: {} ({}s timeout)", "⏳".yellow(), selector, timeout_secs);
        
        while start_time.elapsed() < timeout {
            // Use chromiumoxide to check for element
            match page.find_element(selector).await {
                Ok(_) => {
                    println!("{} Element found: {}", "✓".green(), selector);
                    return Ok(true);
                }
                Err(_) => {
                    // Also try with JavaScript evaluation as backup
                    let check_script = format!(
                        "document.querySelector('{}') !== null",
                        selector
                    );
                    
                    if let Ok(result) = page.evaluate(check_script).await {
                        if let Some(exists) = result.value() {
                            if exists.as_bool().unwrap_or(false) {
                                println!("{} Element found (via JS): {}", "✓".green(), selector);
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).ok();
            sleep(Duration::from_millis(500)).await;
        }
        
        println!("\n{} Timeout waiting for: {}", "❌".red(), selector);
        Ok(false)
    }
}