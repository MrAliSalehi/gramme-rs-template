use std::error::Error;
use std::path::PathBuf;
use async_std::io::WriteExt;
use crate::account::TelegramAccount;

pub async fn prompt(message: &str) -> Result<String, Box<dyn Error>> {
    let mut stdout = async_std::io::stdout();
    stdout.write_all(message.as_bytes()).await?;
    stdout.flush().await?;

    let mut line = String::new();

    async_std::io::stdin().read_line(&mut line).await?;

    Ok(line)
}

pub fn config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("config.json")
}

pub fn config_exists() -> bool {
    config_path().exists()
}

pub fn is_valid(config: &TelegramAccount) -> bool {
    if config.api_hash.is_empty() {
        return false;
    }
    if config.api_hash.len() < 3
        || config.api_id < 3
        || config.phone.len() < 5
    {
        return false;
    }
    true
}