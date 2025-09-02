pub use capabilities::DefaultCapabilitiesBuilder;
use fetch_chromedriver::fetch_chromedriver;
use patch_chromedriver::patch_chromedriver;
use rand::Rng;
use spawn_chromedriver::spawn_chromedriver;
use std::{error::Error, fmt::Display, process::Child};
pub use thirtyfour;
use thirtyfour::WebDriver;
mod capabilities;
mod driver_ext;
mod fetch_chromedriver;
mod get_chrome_version;
mod patch_chromedriver;
mod spawn_chromedriver;
pub use driver_ext::Chrome;

/// Fetches a new ChromeDriver executable and patches it to prevent detection.
/// Returns a WebDriver instance (with default capabilities) and handle to chromedriver process.
pub async fn chrome() -> Result<(WebDriver, Child), Box<dyn std::error::Error + Send + Sync>> {
    chrome_with_capabilities(DefaultCapabilitiesBuilder::new().into_chrome_caps()).await
}

/// Fetches a new ChromeDriver executable and patches it to prevent detection.
/// Returns a WebDriver instance and handle to chromedriver process.
/// If chromedriver fails to start 3 times new chromedriver is redownloaded.
pub async fn chrome_with_capabilities(
    capabilities: thirtyfour::ChromeCapabilities,
) -> Result<(WebDriver, Child), Box<dyn std::error::Error + Send + Sync>> {
    let res = try_start_chrome(capabilities.clone(), 3).await;
    if res.is_ok() {
        return res;
    }
    let os = std::env::consts::OS;
    let chromedriver_executable = match os {
        "linux" => "chromedriver",
        "macos" => "chromedriver",
        "windows" => "chromedriver.exe",
        _ => return Err(UnsupportedOS.into()),
    };
    let _ = std::fs::remove_file(chromedriver_executable);
    let chromedriver_executable = get_patched_chrome_driver_executable()?;
    let _ = std::fs::remove_file(chromedriver_executable);
    try_start_chrome(capabilities, 3).await
}

/// Fetches a new ChromeDriver executable and patches it to prevent detection.
/// Returns a WebDriver instance and handle to chromedriver process.
pub async fn try_start_chrome(
    capabilities: thirtyfour::ChromeCapabilities,
    num_attempts: u8,
) -> Result<(WebDriver, Child), Box<dyn std::error::Error + Send + Sync>> {
    if std::path::Path::new("chromedriver").exists()
        || std::path::Path::new("chromedriver.exe").exists()
    {
        tracing::info!("ChromeDriver already exists!");
    } else {
        tracing::info!("ChromeDriver does not exist! Fetching...");
        fetch_chromedriver().await?;
    }

    // Ensure existing driver matches host architecture on macOS.
    // If not, remove and re-download the correct one.
    if !is_existing_chromedriver_compatible()? {
        tracing::warn!(
            "Existing ChromeDriver appears incompatible with host arch. Re-downloading..."
        );
        let _ = std::fs::remove_file("chromedriver");
        let _ = std::fs::remove_file("chromedriver.exe");
        let _ = std::fs::remove_file("chromedriver_PATCHED");
        let _ = std::fs::remove_file("chromedriver_PATCHED.exe");
        fetch_chromedriver().await?;
    }
    let chromedriver_executable = get_patched_chrome_driver_executable()?;
    if std::path::Path::new(chromedriver_executable).exists() {
        tracing::info!("Detected patched chromedriver executable!");
    } else {
        patch_chromedriver(chromedriver_executable)?;
    }
    tracing::info!("Starting chromedriver...");
    let port: u16 = rand::rng().random_range(2000..5000);
    let mut chrome_driver_handle = spawn_chromedriver(chromedriver_executable, port)?;
    let mut driver = None;
    let mut attempt: u8 = 0u8;
    while driver.is_none() && attempt < num_attempts {
        attempt += 1;
        match WebDriver::new(&format!("http://127.0.0.1:{}", port), capabilities.clone()).await {
            Ok(d) => driver = Some(d),
            Err(_) => tokio::time::sleep(std::time::Duration::from_millis(250)).await,
        }
    }
    let driver = driver.ok_or_else(|| {
        let _ = chrome_driver_handle.kill();
        let _ = chrome_driver_handle.wait();
        DriverCreationFailed
    })?;
    Ok((driver, chrome_driver_handle))
}

fn is_existing_chromedriver_compatible() -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    #[cfg(target_os = "macos")]
    {
        let arch = std::env::consts::ARCH; // "aarch64" or "x86_64"
        let candidate = if std::path::Path::new("chromedriver_PATCHED").exists() {
            "chromedriver_PATCHED"
        } else if std::path::Path::new("chromedriver").exists() {
            "chromedriver"
        } else {
            return Ok(true);
        };
        let output = std::process::Command::new("/usr/bin/file")
            .arg(candidate)
            .output();
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            // Heuristic: ensure the Mach-O header mentions the expected cpu subtype
            let is_arm_expected = arch == "aarch64";
            let mentions_arm64 = stdout.contains("arm64");
            let mentions_x86_64 = stdout.contains("x86_64");
            if is_arm_expected && mentions_x86_64 {
                return Ok(false);
            }
            if !is_arm_expected && mentions_arm64 {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(true)
    }
}

fn get_patched_chrome_driver_executable(
) -> Result<&'static str, Box<dyn std::error::Error + Send + Sync>> {
    let os = std::env::consts::OS;
    let chromedriver_executable = match os {
        "linux" => "chromedriver_PATCHED",
        "macos" => "chromedriver_PATCHED",
        "windows" => "chromedriver_PATCHED.exe",
        _ => return Err(UnsupportedOS.into()),
    };
    Ok(chromedriver_executable)
}

#[derive(Debug)]
struct DriverCreationFailed;

impl Display for DriverCreationFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Driver creation failed.")
    }
}
impl Error for DriverCreationFailed {}

#[derive(Debug)]
struct UnsupportedOS;

impl Display for UnsupportedOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Your OS is not supported.")
    }
}
impl Error for UnsupportedOS {}
