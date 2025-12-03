mod models;
mod profiserve_client;
mod yahoo_finance;
mod sync_service;

use anyhow::Result;
use console::style;
use sync_service::SyncService;

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n{}", style("═".repeat(60)).cyan());
    println!("{}", style("    📈 Profitis Sync Service").cyan().bold());
    println!("{}\n", style("═".repeat(60)).cyan());

    let profiserve_url = std::env::var("PROFISERVE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());
    
    let sync_interval_secs = std::env::var("SYNC_INTERVAL_SECS")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()
        .unwrap_or(60);

    println!("{}", style("Configuration:").bold().underlined());
    println!("  {} {}", 
        style("Profiserve URL:").dim(),
        style(&profiserve_url).cyan()
    );
    println!("  {} {} ({} minutes)", 
        style("Sync interval:").dim(),
        style(format!("{} seconds", sync_interval_secs)).cyan(),
        style(sync_interval_secs / 60).yellow()
    );
    println!();

    let sync_service = SyncService::new(profiserve_url, sync_interval_secs);
    
    sync_service.start().await?;

    Ok(())
}

