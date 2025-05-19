mod cli;
mod scanner;
mod watcher;

use anyhow::Result;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::CliArgs::parse()?;
    let inbox_path = args.get_inbox_path()?;

    println!("Monitoring directory: {}\n", inbox_path.display());

    let existing_files = scanner::scan_and_print_files(&inbox_path)?;

    watcher::watch_inbox(inbox_path.clone()).await?;

    println!("\nFinal file list:");
    for file in existing_files {
        println!("[FINAL] {}", file.display());
    }

    Ok(())
}
