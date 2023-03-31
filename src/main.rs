use std::error::Error;
use async_std::io::WriteExt;
use grammers_client::{Config, InitParams, Client};
use grammers_session::{Session};
use clap::Parser;
use crate::account::TelegramAccount;
use crate::account_manager::*;
use crate::utils::*;

mod utils;
mod account_manager;
mod account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let account = get_tel_account().await.expect("cant get the telegram account");

    let login = Client::connect(Config {
        api_hash: account.api_hash.clone(),
        api_id: account.api_id,
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
        session: Session::load_file_or_create(SESSION_FILE).expect("Failed to create session"),
    }).await;

    if login.is_err() {
        panic!("failed to connect to the telegram");
    }

    let client_handler = login.expect("failed to create client");

    if !client_handler.is_authorized().await.expect("couldnt get authorization status")
    {
        println!("you are not authorized,requesting verification code");

        let signed_in = sign_in_async(&account.phone, account.api_id, &account.api_hash, &client_handler)
            .await;

        check_status_async(&client_handler, signed_in).await?;

        save_session(&client_handler)
    }

    //write code...
    let me = client_handler.get_me().await?;
    println!("logged in as: {}", me.username().unwrap_or(me.id().to_string().as_str()));
    client_handler.send_message(me, "hello from gramme-rs template").await?;

    Ok(())
}

async fn get_tel_account() -> Option<TelegramAccount> {
    let config: TelegramAccount;

    if config_exists() {
        let content = async_std::fs::read_to_string("config.json").await
            .expect("Failed to read config file,");

        config = serde_json::from_str(&content).expect("Failed To parse config,invalid json format.");
    } else {
        println!("config.json not found, expecting arguments from CLI");
        config = TelegramAccount::parse();
        let serialize = serde_json::to_string(&config)
            .expect("cant serialize the config, invalid format!");
        async_std::fs::File::create(config_path()).await
            .expect("cant create the config file")
            .write(serialize.as_bytes()).await
            .expect("cant write in config file");
    }

    if !is_valid(&config) {
        panic!("Invalid config data");
    }
    println!("Account:{},[{}-{}].", config.phone, config.api_hash, config.api_id);
    Some(config)
}