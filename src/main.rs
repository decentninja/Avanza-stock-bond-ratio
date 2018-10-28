/// The main commandline interface for the analysis.

use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
extern crate rpassword;
#[macro_use]
extern crate serde_json;

struct Auth {
    totp: String,
    username: String,
    password: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let auth = if args.len() == 5 && args[1] == "auth" {
        Auth {
            totp: args[2].clone(),
            username: args[3].clone(),
            password: args[4].clone(),
        }
    } else {
        auth()
    };
    let stats = calculate_stats(&auth).unwrap(); // TODO: Exchange with result ? main
    println!("{}", stats.format());
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
        let totp_code = avanza_totp_secret(&totp).unwrap().trim().to_string();
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



fn prompt(what: &str) -> String {
    print!("{}: ", what);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn prompt_hidden(what: &str) -> String {
    let message = format!("{}: ", what);
    let password = rpassword::prompt_password_stdout(&message);
    // If rpassword can't hide the password, just prompt like normal.
    password.unwrap_or_else(|_| prompt("Password"))
}
