#[cfg(test)]
mod tests {
    use thirtyfour::prelude::ElementQueryable;
    use thirtyfour::By;
    use undetected_chromedriver::{chrome, chrome_with_capabilities, DefaultCapabilitiesBuilder};

    #[tokio::test]
    async fn test_headless_detection() {
        let (driver, mut handle) = chrome().await.unwrap();
        driver
            .goto("https://arh.antoinevastel.com/bots/areyouheadless")
            .await
            .unwrap();
        let is_headless = driver.query(By::XPath(r#"//*[@id="res"]/p"#));
        assert_eq!(
            is_headless.first().await.unwrap().text().await.unwrap(),
            "You are not Chrome headless"
        );
        driver.quit().await.unwrap();
        handle.kill().unwrap();
    }

    #[tokio::test]
    async fn test_headless_detection2() {
        let (driver, mut handle) = chrome_with_capabilities(
            DefaultCapabilitiesBuilder::new()
                .set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
                .set_headless(true)
                .into_chrome_caps(),
        )
        .await
        .unwrap();
        driver
            .goto("https://arh.antoinevastel.com/bots/areyouheadless")
            .await
            .unwrap();
        let is_headless = driver.query(By::XPath(r#"//*[@id="res"]/p"#));
        assert_eq!(
            is_headless.first().await.unwrap().text().await.unwrap(),
            "You are not Chrome headless"
        );
        driver.quit().await.unwrap();
        handle.kill().unwrap();
    }
}
