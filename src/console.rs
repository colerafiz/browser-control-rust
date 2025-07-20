use anyhow::Result;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::browser::BrowserController;

pub struct Console {
    browser: Arc<Mutex<BrowserController>>,
    editor: DefaultEditor,
}

impl Console {
    pub fn new(browser: Arc<Mutex<BrowserController>>) -> Result<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self { browser, editor })
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("{}", "🚀 Browser CLI Interactive Console".bold().cyan());
        println!("{}", "Type 'help' for available commands, 'exit' to quit".dimmed());
        println!();

        loop {
            let readline = self.editor.readline("browser> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    self.editor.add_history_entry(line).ok();

                    if line == "exit" || line == "quit" {
                        println!("{}", "Goodbye! 👋".green());
                        break;
                    }

                    if let Err(e) = self.execute_command(line).await {
                        println!("{} {}", "Error:".red().bold(), e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "Use 'exit' to quit".yellow());
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye! 👋".green());
                    break;
                }
                Err(err) => {
                    println!("{} {}", "Error:".red().bold(), err);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn execute_command(&self, input: &str) -> Result<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "help" | "h" => self.show_help(),
            "navigate" | "go" => self.cmd_navigate(args).await,
            "click" => self.cmd_click(args).await,
            "clickat" => self.cmd_click_at(args).await,
            "doubleclickat" => self.cmd_double_click_at(args).await,
            "rightclickat" => self.cmd_right_click_at(args).await,
            "type" => self.cmd_type(args).await,
            "scroll" => self.cmd_scroll(args).await,
            "search" => self.cmd_search(args).await,
            "screenshot" | "ss" => self.cmd_screenshot(args).await,
            "text" => self.cmd_text(args).await,
            "js" | "eval" => self.cmd_javascript(args).await,
            "url" => self.cmd_url().await,
            "title" => self.cmd_title().await,
            "reload" | "refresh" => self.cmd_reload().await,
            "back" => self.cmd_back().await,
            "forward" => self.cmd_forward().await,
            "waitfor" => self.cmd_wait_for(args).await,
            "waitfortext" => self.cmd_wait_for_text(args).await,
            "waitfornav" => self.cmd_wait_for_navigation(args).await,
            "highlight" => self.cmd_highlight(args).await,
            "clear" | "cls" => self.cmd_clear(),
            "status" => self.cmd_status().await,
            "info" => self.cmd_page_info().await,
            "elements" => self.cmd_elements().await,
            "fill" => self.cmd_fill_field(args).await,
            "submit" => self.cmd_submit_form(args).await,
            "ticker" => self.cmd_ticker(args).await,
            "waitenhanced" => self.cmd_wait_enhanced(args).await,
            _ => {
                println!("{} Unknown command: '{}'. Type 'help' for available commands.", 
                    "⚠️".yellow(), command);
                Ok(())
            }
        }
    }

    fn show_help(&self) -> Result<()> {
        println!("{}", "📖 Available Commands:".bold().blue());
        println!();
        
        println!("{}", "Navigation:".bold());
        println!("  {} <url>        Navigate to URL", "navigate, go".cyan());
        println!("  {}              Go back in history", "back".cyan());
        println!("  {}           Go forward in history", "forward".cyan());
        println!("  {}, {}     Reload current page", "reload".cyan(), "refresh".cyan());
        println!();
        
        println!("{}", "Interaction:".bold());
        println!("  {} <selector>     Click an element", "click".cyan());
        println!("  {} <x> <y>        Click at coordinates", "clickat".cyan());
        println!("  {} <x> <y>   Double-click at coordinates", "doubleclickat".cyan());
        println!("  {} <x> <y>    Right-click at coordinates", "rightclickat".cyan());
        println!("  {} <sel> <text>   Type text into element", "type".cyan());
        println!("  {} <dir> [amt]    Scroll (up/down/top/bottom)", "scroll".cyan());
        println!("  {} <query>      Search on current page", "search".cyan());
        println!();
        
        println!("{}", "Information:".bold());
        println!("  {} [selector]     Get text content", "text".cyan());
        println!("  {}               Get current URL", "url".cyan());
        println!("  {}              Get page title", "title".cyan());
        println!("  {}             Check browser status", "status".cyan());
        println!();
        
        println!("{}", "Capture:".bold());
        println!("  {}, {} [file]  Take screenshot", "screenshot".cyan(), "ss".cyan());
        println!();
        
        println!("{}", "JavaScript:".bold());
        println!("  {}, {} <code>    Execute JavaScript", "js".cyan(), "eval".cyan());
        println!();
        
        println!("{}", "Waiting:".bold());
        println!("  {} <sel> [s]   Wait for element to appear", "waitfor".cyan());
        println!("  {} <text> [s] Wait for text to appear", "waitfortext".cyan());
        println!("  {} [s]        Wait for navigation", "waitfornav".cyan());
        println!();
        
        println!("{}", "Debugging:".bold());
        println!("  {} <selector>    Highlight element temporarily", "highlight".cyan());
        println!("  {}              Get detailed page information", "info".cyan());
        println!("  {}           List interactive elements", "elements".cyan());
        println!();
        
        println!("{}", "Form Handling:".bold());
        println!("  {} <sel> <val>    Robust form field filling", "fill".cyan());
        println!("  {} [selector]     Submit form", "submit".cyan());
        println!();
        
        println!("{}", "Monitoring:".bold());
        println!("  {} [sel] [interval] [max] Monitor page changes", "ticker".cyan());
        println!("  {} <sel> [timeout] Enhanced element waiting", "waitenhanced".cyan());
        println!();
        
        println!("{}", "Utility:".bold());
        println!("  {}, {}         Clear screen", "clear".cyan(), "cls".cyan());
        println!("  {}, {}           Show this help", "help".cyan(), "h".cyan());
        println!("  {}, {}           Exit console", "exit".cyan(), "quit".cyan());
        println!();
        
        Ok(())
    }

    async fn cmd_navigate(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: navigate <url>", "⚠️".yellow());
            return Ok(());
        }
        
        let url = args.join(" ");
        let mut browser = self.browser.lock().await;
        browser.navigate(&url).await
    }

    async fn cmd_click(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: click <selector>", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.click(selector).await
    }

    async fn cmd_click_at(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            println!("{} Usage: clickat <x> <y>", "⚠️".yellow());
            return Ok(());
        }
        
        let x = args[0].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid X coordinate"))?;
        let y = args[1].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid Y coordinate"))?;
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.click_at_coordinates(x, y).await
    }

    async fn cmd_double_click_at(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            println!("{} Usage: doubleclickat <x> <y>", "⚠️".yellow());
            return Ok(());
        }
        
        let x = args[0].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid X coordinate"))?;
        let y = args[1].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid Y coordinate"))?;
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.double_click_at_coordinates(x, y).await
    }

    async fn cmd_right_click_at(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            println!("{} Usage: rightclickat <x> <y>", "⚠️".yellow());
            return Ok(());
        }
        
        let x = args[0].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid X coordinate"))?;
        let y = args[1].parse::<f64>()
            .map_err(|_| anyhow::anyhow!("Invalid Y coordinate"))?;
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.right_click_at_coordinates(x, y).await
    }

    async fn cmd_type(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            println!("{} Usage: type <selector> <text>", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let text = args[1..].join(" ");
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.type_text(selector, &text).await
    }

    async fn cmd_scroll(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: scroll <up|down|top|bottom> [amount]", "⚠️".yellow());
            return Ok(());
        }
        
        let direction = args[0];
        let amount = args.get(1).and_then(|s| s.parse().ok());
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.scroll(direction, amount).await
    }

    async fn cmd_search(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: search <query>", "⚠️".yellow());
            return Ok(());
        }
        
        let query = args.join(" ");
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.search(&query).await
    }

    async fn cmd_screenshot(&self, args: &[&str]) -> Result<()> {
        let filename = args.get(0).copied();
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.screenshot(filename).await?;
        Ok(())
    }

    async fn cmd_text(&self, args: &[&str]) -> Result<()> {
        let selector = args.get(0).copied();
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        let text = browser.get_text(selector).await?;
        println!("{}", text.cyan());
        Ok(())
    }

    async fn cmd_javascript(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: js <javascript_code>", "⚠️".yellow());
            return Ok(());
        }
        
        let code = args.join(" ");
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.execute_javascript(&code).await
    }

    async fn cmd_url(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        let url = browser.get_url().await?;
        println!("{} {}", "URL:".bold(), url.cyan());
        Ok(())
    }

    async fn cmd_title(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        let title = browser.get_title().await?;
        println!("{} {}", "Title:".bold(), title.cyan());
        Ok(())
    }

    async fn cmd_reload(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.reload().await
    }

    async fn cmd_back(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.go_back().await
    }

    async fn cmd_forward(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.go_forward().await
    }

    async fn cmd_wait_for(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: waitfor <selector> [timeout]", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let timeout = args.get(1).and_then(|s| s.parse().ok());
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.wait_for_selector(selector, timeout).await
    }

    async fn cmd_wait_for_text(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: waitfortext <text> [timeout]", "⚠️".yellow());
            return Ok(());
        }
        
        // Check if last argument is a number (timeout)
        let (text, timeout) = if args.len() > 1 {
            if let Ok(timeout_secs) = args.last().unwrap().parse::<u64>() {
                let text_parts = &args[..args.len() - 1];
                (text_parts.join(" "), Some(timeout_secs))
            } else {
                (args.join(" "), None)
            }
        } else {
            (args.join(" "), None)
        };
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.wait_for_text(&text, timeout).await
    }

    async fn cmd_wait_for_navigation(&self, args: &[&str]) -> Result<()> {
        let timeout = args.get(0).and_then(|s| s.parse().ok());
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.wait_for_navigation(timeout).await
    }

    async fn cmd_highlight(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: highlight <selector>", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.highlight_element(selector).await
    }

    fn cmd_clear(&self) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "🚀 Browser CLI Interactive Console".bold().cyan());
        println!("{}", "Type 'help' for available commands, 'exit' to quit".dimmed());
        println!();
        Ok(())
    }

    async fn cmd_status(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        let status = browser.get_status().await?;
        println!("{}", status);
        Ok(())
    }

    async fn cmd_page_info(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        let info = browser.get_concise_page_info().await?;
        println!("{}", info);
        Ok(())
    }

    async fn cmd_elements(&self) -> Result<()> {
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        
        let elements_info = browser.get_interactive_elements().await?;
        println!("{}", elements_info);
        
        Ok(())
    }

    async fn cmd_fill_field(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            println!("{} Usage: fill <selector> <value>", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let value = args[1..].join(" ");
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.fill_form_field(selector, &value).await
    }

    async fn cmd_submit_form(&self, args: &[&str]) -> Result<()> {
        let selector = args.get(0).copied();
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        browser.submit_form(selector).await
    }

    async fn cmd_ticker(&self, args: &[&str]) -> Result<()> {
        let selector = args.get(0).copied();
        let interval = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(2);
        let max_iterations = args.get(2).and_then(|s| s.parse::<u64>().ok());
        
        if interval == 0 {
            println!("{} Interval must be greater than 0 seconds", "⚠️".yellow());
            return Ok(());
        }
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        
        if let Some(sel) = selector {
            println!("{} Starting ticker for selector: {}", "⏱️".cyan(), sel);
        } else {
            println!("{} Starting page monitoring ticker", "⏱️".cyan());
        }
        
        browser.start_ticker(selector, interval, max_iterations).await
    }

    async fn cmd_wait_enhanced(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("{} Usage: waitenhanced <selector> [timeout_seconds]", "⚠️".yellow());
            return Ok(());
        }
        
        let selector = args[0];
        let timeout = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(10);
        
        let mut browser = self.browser.lock().await;
        browser.init().await?;
        
        match browser.wait_for_element_enhanced(selector, timeout).await {
            Ok(found) => {
                if found {
                    println!("{} Element ready for interaction", "✅".green());
                } else {
                    println!("{} Element not found within timeout", "❌".red());
                }
            }
            Err(e) => {
                println!("{} Wait error: {}", "⚠️".yellow(), e);
            }
        }
        
        Ok(())
    }
}