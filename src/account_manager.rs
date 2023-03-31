use std::error::Error;
use crate::utils::prompt;
use grammers_client::types::User;
use grammers_client::{Client, SignInError};

pub const SESSION_FILE: &str = "gramme_rs.session";

pub fn save_session(client_handler: &Client) {
    match client_handler.session().save_to_file(SESSION_FILE) {
        Ok(_) => {
            println!("session saved to: {}", SESSION_FILE)
        }
        Err(e) => {
            println!(
                "NOTE: failed to save the session[{}],you will sign out when program stops working",
                e
            );
        }
    }
}

pub async fn check_status_async(client_handler: &Client, signed_in: Result<User, SignInError>) -> Result<(), Box<dyn Error>> {
    match signed_in {
        Err(SignInError::PasswordRequired(password_token)) => {
            let hint = password_token.hint().unwrap_or("None");
            let msg = format!("Enter the password (hint {}): ", &hint);
            let password = prompt(msg.as_str()).await.expect("failed to get the password");
            client_handler.check_password(password_token, password.trim()).await?;
        }
        Ok(user) => {
            println!("logged in with user:{},id:{}", user.username().unwrap(), user.id());
        }
        Err(e) => panic!("{}", e),
    };
    Ok(())
}

pub async fn sign_in_async(phone: &str, id: i32, hash: &str, client_handler: &Client) -> Result<User, SignInError> {
    let login_token = client_handler.request_login_code(phone, id, hash)
        .await.expect("failed to send code");
    let code = prompt("Enter Code:").await.expect("failed to get the code");
    return client_handler.sign_in(&login_token, &code).await;
}