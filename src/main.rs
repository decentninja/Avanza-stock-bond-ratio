/// The main commandline interface for the analysis.
use std::env;
use std::io::{self, Write};
extern crate rpassword;
extern crate serde_json;

mod analysis;
mod avanza;

fn main() -> Result<(), serde_json::Value> {
    let args: Vec<String> = env::args().collect();
    let auth = if args[1] == "auth" {
        if args.len() != 5 {
            return Err(serde_json::json!("You need to pass 4 arguments to auth!"));
        }
        Auth {
            totp: args[2].clone(),
            username: args[3].clone(),
            password: args[4].clone(),
        }
    } else {
        auth()
    };
    let stats = analysis::calculate_stats(&auth)?;
    println!("{}", stats.format());
    Ok(())
}

/// Avanza Credentials
pub struct Auth {
    totp: String,
    username: String,
    password: String,
}

/// Commandline guide to setup credentials and activate a totp two factor code on the avanza website.
fn auth() -> Auth {
    let first_time = prompt("Is this your first time using this application?\nI.e should we generate a TOTP code? (y/n)");
    let totp = if first_time == "y" {
        let help_message = r#"
1. Go to Mina Sidor > Profil > Sajtinställningar > Tvåfaktorsinloggning
2. Click "Aktivera" on the next screen.
3. Select "Annan app för tvåfaktorsinloggning".
4. Click "Kan du inte scanna QR-koden?" to reveal your TOTP Secret.
5. Copy that keep it safe for future login.
6. Also enter it here (don't close the website yet):"#;
        println!("{}", help_message);
        let totp = prompt("TOTP");
        let totp_code = avanza::totp_secret(&totp).unwrap().trim().to_string();
        println!(
            r#"
7. Copy {} into the "Fyll i engångskoden från appen." field.
8. Your done!"#,
            totp_code
        );
        totp
    } else {
        prompt("TOTP")
    };
    let username = prompt("Username");
    let password = prompt_hidden("Password");
    println!(
        "Next time you can login by running \"avanza-additional-analysis auth {} {} [password]\"",
        totp, username
    );
    Auth {
        totp,
        username,
        password,
    }
}

/// Ask the user a question with a visible answer. For sensitive questions such as passwords, consider prompt_hidden.
fn prompt(what: &str) -> String {
    print!("{}: ", what);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

/// Try to ask the user a question with a hidden answer, if the terminal does not support this, run prompt.
fn prompt_hidden(what: &str) -> String {
    let message = format!("{}: ", what);
    let password = rpassword::prompt_password_stdout(&message);
    password.unwrap_or_else(|_| prompt("Password"))
}

#[cfg(test)]
mod tests {
    // This file should only contain interfacing logic, not business logic. 
    // Interface logic should be tested manually before git commit.
}