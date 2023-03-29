use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelegramAccount {
    pub api_hash: String,
    pub api_id: i32,
    pub phone: String
}