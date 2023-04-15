mod prelude;

use crate::prelude::*;

use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use std::{env::var, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let (api_hash, api_id, session_file) = (
        Arc::new(var("API_HASH")?),
        var("API_ID")?.parse::<i32>()?,
        var("SESSION_FILE")?,
    );

    let client = Client::connect(Config {
        api_hash: api_hash.to_string(),
        api_id: api_id,
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
        session: Session::load_file_or_create(&session_file)?,
    })
    .await?;

    if !client.is_authorized().await? {
        let (login_token, code) = (
            client
                .request_login_code(&var("PHONE")?, api_id, &api_hash)
                .await?,
            prompt("verification code: ").await?,
        );

        if let Err(e) = client.sign_in(&login_token, &code).await {
            let SignInError::PasswordRequired(password_token) = e else {
                return Err(e.into());
            };

            let password = prompt(&format!(
                "2FA password (hint: {}): ",
                password_token.hint().unwrap_or_default()
            ))
            .await?;
            client.check_password(password_token, password).await?;
        };
        client.session().save_to_file(&session_file)?
    }

    let me = client.get_me().await?;
    tracing::info!(
        "logged in as {} (ID: {})",
        me.username().unwrap_or_default(),
        me.id()
    );
    client
        .send_message(me, "hello from gramme.rs template")
        .await?;

    Ok(())
}
