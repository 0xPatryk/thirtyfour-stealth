use std::{error::Error, fmt::Display};

use crate::get_chrome_version::get_chrome_version;

pub async fn fetch_chromedriver() -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = std::env::consts::OS;
    let client = reqwest::Client::new();

    let installed_version = get_chrome_version(os).await?;
    let chromedriver_url: String;
    if installed_version.as_str() >= "114" {
        // Fetch the correct version
        let url = "https://googlechromelabs.github.io/chrome-for-testing/latest-versions-per-milestone.json";
        let resp = client.get(url).send().await?;
        let body = resp.bytes().await?;
        let json = serde_json::from_slice::<serde_json::Value>(&body)?;
        let version = json["milestones"][installed_version]["version"]
            .as_str()
            .ok_or(ChromeDriverFetchError)?;

        // Fetch the chromedriver binary
        chromedriver_url = match os {
            "linux" => format!(
                "https://storage.googleapis.com/chrome-for-testing-public/{}/{}/{}",
                version, "linux64", "chromedriver-linux64.zip"
            ),
            "macos" => format!(
                "https://storage.googleapis.com/chrome-for-testing-public/{}/{}/{}",
                version, "mac-x64", "chromedriver-mac-x64.zip"
            ),
            "windows" => format!(
                "https://storage.googleapis.com/chrome-for-testing-public/{}/{}/{}",
                version, "win64", "chromedriver-win64.zip"
            ),
            _ => panic!("Unsupported OS!"),
        };
    } else {
        let resp = client
            .get(format!(
                "https://chromedriver.storage.googleapis.com/LATEST_RELEASE_{}",
                installed_version
            ))
            .send()
            .await?;
        let body = resp.text().await?;
        chromedriver_url = match os {
            "linux" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
                body
            ),
            "windows" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_win32.zip",
                body
            ),
            "macos" => format!(
                "https://chromedriver.storage.googleapis.com/{}/chromedriver_mac64.zip",
                body
            ),
            _ => panic!("Unsupported OS!"),
        };
    }

    let resp = client.get(&chromedriver_url).send().await?;
    let body = resp.bytes().await?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(body))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.mangled_name();
        if let Some(file_name) = file_name.file_name() {
            let file_name = file_name.to_string_lossy();
            if file.name().ends_with("/")
                || file_name != "chromedriver" && file_name != "chromedriver.exe"
            {
                continue;
            }
            let mut out_file = std::fs::File::create(file_name.as_ref())?;
            std::io::copy(&mut file, &mut out_file)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct ChromeDriverFetchError;

impl Display for ChromeDriverFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No chromedriver version was found.")
    }
}

impl Error for ChromeDriverFetchError {}
