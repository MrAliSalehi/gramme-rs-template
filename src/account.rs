use clap::{ColorChoice, Parser};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Deserialize, Serialize)]
#[command(color=ColorChoice::Always,disable_help_flag=true)]
pub struct TelegramAccount {
    #[arg(short = 'h',long, name="hash")]
    pub api_hash: String,

    #[arg(short = 'i',long, name="id")]
    pub api_id: i32,

    #[arg(short = 'p',long, name="phone")]
    pub phone: String
}