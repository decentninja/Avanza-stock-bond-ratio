/// Interactive interface
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

/// Generate a totp (Time-based One-time Password).
pub fn totp_secret(totp: &str) -> Result<String, String> {
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

/// Talk spawns the node script and continously talks to it over stdin/stdout/stderr, sending commands.
pub struct Talk {
    child: Child,
}

impl Talk {
    pub fn new(auth: &super::Auth) -> Result<Self, String> {
        totp_secret(&auth.totp)?;
        let mut child = Command::new("node")
            .args(&[
                "index.js",
                "talk",
                &auth.totp,
                &auth.username,
                &auth.password,
            ]).stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to run nodejs!");
        {
            let mut buf = String::new();
            let mut stdout = BufReader::new(child.stdout.as_mut().unwrap());
            stdout.read_line(&mut buf).unwrap();
            if buf.trim() != "ready" {
                buf.clear();
                let mut stderr = BufReader::new(child.stderr.as_mut().unwrap());
                stderr.read_line(&mut buf).unwrap();
                return Err(buf);
            }
        }
        Ok(Talk { child })
    }
    pub fn command(&mut self, arguments: &[&str]) -> Result<serde_json::Value, serde_json::Value> {
        let mut buf = String::new();
        let mut stdout = BufReader::new(self.child.stdout.as_mut().unwrap());
        self.child
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
}

mod test {
    use super::*;

    fn totp() {}
}
