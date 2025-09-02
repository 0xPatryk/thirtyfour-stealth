#[cfg(test)]
mod tests {
    use thirtyfour::prelude::ElementQueryable;
    use thirtyfour::By;
    use undetected_chromedriver::{chrome, Chrome};

    #[tokio::test]
    async fn test_cloudflare() {
        let (driver, mut handle) = chrome().await.unwrap();
        driver.goto("https://nowsecure.nl").await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        driver.enter_frame(0).await.unwrap();
        let passed = driver.query(By::Css("span#success-text"));
        assert_eq!(
            passed.first().await.unwrap().text().await.unwrap(),
            "Success!"
        );
        driver.quit().await.unwrap();
        handle.kill().unwrap();
    }
}
