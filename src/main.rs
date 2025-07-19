mod browser;

use anyhow::Result;
use browser::BrowserController;
use clap::{Parser, Subcommand};
use colored::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(name = "browser-cli")]
#[command(about = "Command line browser automation tool")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "go", about = "Navigate to a URL")]
    Navigate {
        #[arg(help = "URL to navigate to")]
        url: String,
    },
    #[command(about = "Click an element by CSS selector")]
    Click {
        #[arg(help = "CSS selector of element to click")]
        selector: String,
    },
    #[command(about = "Type text into an element")]
    Type {
        #[arg(help = "CSS selector of input element")]
        selector: String,
        #[arg(help = "Text to type")]
        text: String,
    },
    #[command(about = "Scroll the page")]
    Scroll {
        #[arg(help = "Direction to scroll (up|down|top|bottom)")]
        direction: String,
        #[arg(help = "Amount to scroll in pixels (optional)")]
        amount: Option<i32>,
    },
    #[command(about = "Search for text on the current page")]
    Search {
        #[arg(help = "Search query")]
        query: String,
    },
    #[command(about = "Take a screenshot of the current page")]
    Screenshot {
        #[arg(help = "Optional filename for screenshot")]
        filename: Option<String>,
    },
    #[command(about = "Get text content from an element or page info")]
    Text {
        #[arg(help = "CSS selector (optional - gets page info if omitted)")]
        selector: Option<String>,
    },
    #[command(about = "Close the browser")]
    Close,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let browser = Arc::new(Mutex::new(BrowserController::new()));
    
    // Set up signal handling for graceful shutdown
    let browser_clone = Arc::clone(&browser);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("{}", "\nReceived interrupt signal, closing browser...".yellow());
        let mut browser = browser_clone.lock().await;
        browser.close().await.ok();
        std::process::exit(0);
    });

    match cli.command {
        Commands::Navigate { url } => {
            let mut browser = browser.lock().await;
            browser.navigate(&url).await?;
        }
        Commands::Click { selector } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            browser.click(&selector).await?;
        }
        Commands::Type { selector, text } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            browser.type_text(&selector, &text).await?;
        }
        Commands::Scroll { direction, amount } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            browser.scroll(&direction, amount).await?;
        }
        Commands::Search { query } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            browser.search(&query).await?;
        }
        Commands::Screenshot { filename } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            browser.screenshot(filename.as_deref()).await?;
        }
        Commands::Text { selector } => {
            let mut browser = browser.lock().await;
            browser.init().await?;
            let text = browser.get_text(selector.as_deref()).await?;
            println!("{}", text.cyan());
        }
        Commands::Close => {
            let mut browser = browser.lock().await;
            browser.close().await?;
        }
    }

    Ok(())
}
