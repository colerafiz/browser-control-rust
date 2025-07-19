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
            "clear" | "cls" => self.cmd_clear(),
            "status" => self.cmd_status().await,
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

    fn cmd_clear(&self) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "🚀 Browser CLI Interactive Console".bold().cyan());
        println!("{}", "Type 'help' for available commands, 'exit' to quit".dimmed());
        println!();
        Ok(())
    }

    async fn cmd_status(&self) -> Result<()> {
        let browser = self.browser.lock().await;
        if browser.is_initialized() {
            println!("{} Browser is running and ready", "✅".green());
        } else {
            println!("{} Browser not initialized", "⚠️".yellow());
        }
        Ok(())
    }
}