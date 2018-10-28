/// Interactive interface

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

fn avanza_totp_secret(totp: &str) -> Result<String, String> {
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

fn avanza_talk(auth: &Auth) -> Result<Child, String> {
    avanza_totp_secret(&auth.totp)?;
    let mut child = Command::new("node")
        .args(&[
            "index.js",      // TODO: Inline this with inclue_str instead so that we can cargo install it.
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
    Ok(child)
}