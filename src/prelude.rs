use std::io::Write;

pub async fn prompt(message: &str) -> eyre::Result<String> {
    let mut stdout = std::io::stdout();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    Ok(line.trim().to_string())
}
