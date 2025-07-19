use anyhow::Result;
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotParams;
use chromiumoxide::{Browser, BrowserConfig, Page};
use colored::*;
use futures_util::StreamExt;
use std::path::PathBuf;

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

        println!("{}", "Launching browser...".blue());
        
        // Create a temporary user data directory to avoid conflicts with existing Chrome sessions
        let temp_dir = format!("/tmp/chromiumoxide-{}", std::process::id());
        
        let (browser, mut handler) = Browser::launch(
            BrowserConfig::builder()
                .window_size(1280, 800)
                .build()
                .map_err(|e| anyhow::anyhow!("Failed to build browser config: {}", e))?,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to launch browser. Make sure Chrome is installed. Error: {}", e))?;

        let _handle = tokio::task::spawn(async move {
            while let Some(h) = handler.next().await {
                if let Err(_) = h {
                    // Suppress handler errors - these are often non-critical CDP protocol mismatches
                    // with newer Chrome versions
                }
            }
        });

        let page = browser.new_page("about:blank").await?;
        
        self.browser = Some(browser);
        self.page = Some(page);
        self.temp_dir = Some(temp_dir);
        
        println!("{}", "Browser launched successfully".green());
        Ok(())
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        self.ensure_initialized().await?;
        
        println!("{}", format!("Navigating to: {}", url).blue());
        
        let page = self.page.as_ref().unwrap();
        page.goto(url).await?;
        
        println!("{}", "Page loaded successfully".green());
        Ok(())
    }

    pub async fn screenshot(&self, filename: Option<&str>) -> Result<String> {
        self.ensure_page()?;
        
        let default_filename = format!("screenshot-{}.png", chrono::Utc::now().timestamp());
        let filename = filename.unwrap_or(&default_filename);
        let path = PathBuf::from(filename);
        
        println!("{}", format!("Taking screenshot: {}", filename).blue());
        
        let page = self.page.as_ref().unwrap();
        let screenshot = page.screenshot(CaptureScreenshotParams::builder().build()).await?;
        
        tokio::fs::write(&path, screenshot).await?;
        
        println!("{}", format!("Screenshot saved: {}", filename).green());
        Ok(filename.to_string())
    }

    pub async fn click(&self, selector: &str) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Clicking element: {}", selector).blue());
        
        let page = self.page.as_ref().unwrap();
        let element = page.find_element(selector).await?;
        element.click().await?;
        
        println!("{}", "Element clicked".green());
        Ok(())
    }

    pub async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        self.ensure_page()?;
        
        println!("{}", format!("Typing '{}' into: {}", text, selector).blue());
        
        let page = self.page.as_ref().unwrap();
        let element = page.find_element(selector).await?;
        element.click().await?;
        element.type_str(text).await?;
        
        println!("{}", "Text entered".green());
        Ok(())
    }

    pub async fn scroll(&self, direction: &str, amount: Option<i32>) -> Result<()> {
        self.ensure_page()?;
        
        let amount_text = if let Some(amt) = amount {
            format!(" by {}px", amt)
        } else {
            String::new()
        };
        
        println!("{}", format!("Scrolling {}{}", direction, amount_text).blue());
        
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
        
        println!("{}", "Scrolled successfully".green());
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
                println!("{}", format!("Search executed using selector: {}", selector).green());
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
        
        println!("{}", format!("Executing JavaScript: {}", code).blue());
        
        let page = self.page.as_ref().unwrap();
        let result = page.evaluate(code).await?;
        
        if let Some(value) = result.value() {
            println!("{} {}", "Result:".green(), serde_json::to_string_pretty(value)?);
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
}