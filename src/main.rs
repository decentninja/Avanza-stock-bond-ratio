use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
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
            password: args[4].clone(),
        }
    } else {
        auth()
    };
    let stats = calculate_stats(&auth).unwrap();
    println!("{}", stats.format());
}

#[derive(Default)]
struct Stats {
    types: HashMap<String, f64>,
}

impl Stats {
    fn format(&self) -> String {
        let total: f64 = self.types.values().sum();
        let lines = self.types
            .iter()
            .map(|(name, value)| {
                format!(
                    "{:10}  {:>10.1}  {:>4.1}%",
                    name,
                    value,
                    100. * value / total
                )
            }).collect::<Vec<String>>()
            .join("\n");
        format!(
            "Your portfolio consists of\n{}\nTotal: {:>10.1}",
            lines, total
        )
    }

    fn track(&mut self, name: String, value: f64) {
        *self.types.entry(name).or_default() += value;
    }
}

fn calculate_stats(auth: &Auth) -> Result<Stats, serde_json::Value> {
    let mut child = avanza_talk(&auth).unwrap();
    let positions = talk_command(&mut child, &["getpositions"])?;
    let mut stats = Stats::default();
    for category in positions["instrumentPositions"].as_array().unwrap() {
        match category["instrumentType"].as_str().unwrap() {
            "STOCK" => {
                for position in category["positions"].as_array().unwrap() {
                    let value = position["value"].as_f64().unwrap();
                    stats.track("stock".to_string(), value);
                }
            }
            "FUND" => {
                for position in category["positions"].as_array().unwrap() {
                    let value = position["value"].as_f64().unwrap();
                    let orderbookid = position["orderbookId"].as_str().unwrap();
                    let instrument = talk_command(&mut child, &["getinstrument", "FUND", orderbookid])?;
                    stats.track(instrument["type"].as_str().unwrap().to_string(), value);
                }
            }
            instrument_type => eprintln!("Not handled case {}", instrument_type),
        }
    }
    Ok(stats)
}

fn talk_command(
    child: &mut Child,
    arguments: &[&str],
) -> Result<serde_json::Value, serde_json::Value> {
    let mut buf = String::new();
    let mut stdout = BufReader::new(child.stdout.as_mut().unwrap());
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(format!("{}\n", arguments.join(" ")).as_bytes())
        .unwrap();
    stdout.read_line(&mut buf).unwrap();
    let mut result: serde_json::Value = serde_json::from_str(&buf).unwrap();
    match result["type"].as_str().unwrap() {
        "error" => Err(result["description"].take()),
        _ => Ok(result["result"].take()),
    }
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
    Auth {
        totp,
        username,
        password,
    }
}
type AvanzaResult = Result<String, String>;

fn avanza_totp_secret(totp: &str) -> AvanzaResult {
    let result = Command::new("node")
        .args(&["index.js", "totp", totp])
        .output()
        .expect("Executable to exist");
    let err = String::from_utf8(result.stderr).expect("Unicode lol");
    if !err.is_empty() {
        return Err(err);
    }
    Ok(String::from_utf8(result.stdout).expect("Unicode lol"))
}

fn avanza_talk(auth: &Auth) -> Result<Child, std::io::Error> {
    avanza_totp_secret(&auth.totp).unwrap();
    let mut child = Command::new("node")
        .args(&[
            "index.js",
            "talk",
            &auth.totp,
            &auth.username,
            &auth.password,
        ]).stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut buf = String::new();
        let mut stdout = BufReader::new(child.stdout.as_mut().unwrap());
        stdout.read_line(&mut buf).unwrap();
        if buf.trim() != "ready" {
            eprintln!("Node not ready?");
        }
    }
    Ok(child)
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
