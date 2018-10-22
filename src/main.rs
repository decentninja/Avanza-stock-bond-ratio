use std::process::Command;
use std::io::{self, Write};
use std::env;
extern crate rpassword;


fn main() {
    let args: Vec<String> = env::args().collect();
    let (totp, username, password) = if args.len() == 5 && args[1] == "auth" {
        (args[2].clone(), args[3].clone(), args[4].clone())
    } else {
        auth()
    };
    let positions = avanza_positions(&totp, &username, &password);
    println!("Positions: {:#?}", positions);
}

fn auth() -> (String, String, String) {
    let first_time = prompt("Is this your first time using this application (y/n)");
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
        println!(r#"
7. Copy {} into the "Fyll i engångskoden från appen." field.
8. Your done!"#, totp_code);
        totp
    } else {
        prompt("TOTP")
    };
    let username = prompt("Username");
    let password = prompt_hidden("Password");
    (totp, username, password)
}
type AvanzaResult = Result<String, String>;

fn avanza_totp_secret(totp: &str) -> AvanzaResult {
    avanza_command("totp", vec![totp])
}

fn avanza_positions(totp: &str, username: &str, password: &str) -> AvanzaResult {
    avanza_totp_secret(&totp).unwrap();
    avanza_command("positions", vec![totp, username, password])
}

fn avanza_command(command: &str, arguments: Vec<&str>) -> AvanzaResult {
    let mut arguments = arguments;
    arguments.insert(0, "index.js");
    arguments.insert(1, command);
    let result = Command::new("node")
        .args(arguments.as_slice())
        .output().expect("Executable to exist");
    let err = String::from_utf8(result.stderr).expect("Unicode lol");
    if err.len() != 0 {
        return Err(err)
    }
    Ok(String::from_utf8(result.stdout).expect("Unicode lol"))
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
    let password = password.unwrap_or_else(|_| prompt("Password"));
    password
}