use std::process::{Command, Child, Stdio};
use std::io::{self, Write, BufReader, BufRead};
use std::env;
extern crate rpassword;
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
            password: args[4].clone() 
        }
    } else {
        auth()
    };
    let mut child = avanza_talk(&auth).unwrap();
    println!("Talk got back: {}", talk_command(&mut child));
}

// TODO: Replace string return value with parsed json
fn talk_command(child: &mut Child) -> Result<serde_json::Value, serde_json::Value> {
    let mut buf = String::new();
    let mut stdout = BufReader::new(child.stdout.as_mut().unwrap());
    stdout.read_line(&mut buf).unwrap();
    if buf.trim() != "ready" {
        eprintln!("Node not ready?");
    }
    buf.clear();
    child.stdin.as_mut().unwrap().write_all("getpositions\n".as_bytes()).unwrap();
    stdout.read_line(&mut buf).unwrap();
    let result = serde_json::from_str(&buf).unwrap();
    if result.get("type")
}

fn auth() -> Auth {
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
    Auth {
        totp,
        username,
        password
    }
}
type AvanzaResult = Result<String, String>;

fn avanza_totp_secret(totp: &str) -> AvanzaResult {
    avanza_command("totp", vec![totp])
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

fn avanza_talk(auth: &Auth) -> Result<Child, std::io::Error> {
    avanza_totp_secret(&auth.totp).unwrap();
    Command::new("node")
        .args(&["index.js", "talk", &auth.totp, &auth.username, &auth.password])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
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