use std::{error::Error, process::Child, time::Duration};

use thirtyfour::{ChromeCapabilities, WebDriver};

use crate::{chrome, chrome_with_capabilities};

#[async_trait::async_trait]
pub trait Chrome: Sized {
    /// Initializes chromedriver with default capabilities.
    ///
    /// Panics if initialization failed!
    /// For non-panicking version use [`chrome`] function.
    async fn new() -> (Self, Child);
    /// Initializes chromedriver with specified capabilities.
    ///
    /// Panics if initialization failed!
    /// For non-panicking version use [`chrome_with_capabilities`] function.
    async fn new_with_capabilities(capabilities: ChromeCapabilities) -> (WebDriver, Child);
    async fn goto(&self, url: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl Chrome for WebDriver {
    async fn new() -> (WebDriver, Child) {
        chrome().await.expect("Failed to initialize chromedriver.")
    }

    async fn new_with_capabilities(capabilities: ChromeCapabilities) -> (WebDriver, Child) {
        chrome_with_capabilities(capabilities)
            .await
            .expect("Failed to initialize chromedriver.")
    }

    async fn goto(&self, url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.execute(&format!(r#"window.open("{}", "_blank");"#, url), vec![])
            .await?;

        tokio::time::sleep(Duration::from_secs(3)).await;

        let first_window = self
            .windows()
            .await?
            .first()
            .expect("Unable to get first windows")
            .clone();
        self.switch_to_window(first_window).await?;
        self.close_window().await?;

        let last_window = self
            .windows()
            .await?
            .last()
            .expect("Unable to get last windows")
            .clone();
        self.switch_to_window(last_window).await?;

        Ok(())
    }
}
