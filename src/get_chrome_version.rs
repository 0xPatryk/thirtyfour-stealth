use std::process::Command;

pub async fn get_chrome_version(os: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("Getting installed Chrome version...");
    let command = match os {
        "linux" => Command::new("/usr/bin/google-chrome")
            .arg("--version")
            .output()?,
        "macos" => Command::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
            .arg("--version")
            .output()?,
        "windows" => Command::new("powershell")
            .arg("-c")
            .arg("(Get-Item 'C:/Program Files/Google/Chrome/Application/chrome.exe').VersionInfo")
            .output()?,
        _ => panic!("Unsupported OS!"),
    };
    let output = String::from_utf8(command.stdout)?;

    let version = output
        .lines()
        .flat_map(|line| line.chars().filter(|&ch| ch.is_ascii_digit()))
        .take(3)
        .collect::<String>();

    tracing::info!("Currently installed Chrome version: {}", version);
    Ok(version)
}
