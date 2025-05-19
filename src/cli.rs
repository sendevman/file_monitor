use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "FileMonitor")]
#[command(about = "Monitors a folder for file changes")]
pub struct CliArgs {
    #[arg(short = 'd', long = "dir")]
    pub dir: Option<String>,
}

impl CliArgs {
    pub fn parse() -> Result<Self> {
        Ok(Parser::parse())
    }

    pub fn get_inbox_path(&self) -> Result<PathBuf> {
        let default_path = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?
            .join("inbox");

        let resolved_path = self
            .dir
            .as_ref()
            .map(|s| PathBuf::from(s))
            .unwrap_or(default_path);

        if !resolved_path.exists() {
            std::fs::create_dir_all(&resolved_path)?;
        }

        Ok(resolved_path)
    }
}
