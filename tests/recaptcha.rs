#[cfg(test)]
mod tests {
    use thirtyfour::prelude::{ElementQueryable, ElementWaitable};
    use thirtyfour::By;
    use undetected_chromedriver::chrome;

    async fn get_score(driver: &thirtyfour::WebDriver) -> Option<f32> {
        driver
            .goto("https://recaptcha-demo.appspot.com/recaptcha-v3-request-scores.php")
            .await
            .unwrap();
        //let button = driver
        //    .query(By::XPath(r#"//*[@id="recaptcha-steps"]/li[2]/button[2]"#))
        //    .first()
        //    .await
        //    .unwrap();
        //button.wait_until().clickable().await.unwrap();
        //button.click().await.unwrap();
        let response = driver.query(By::Css("pre.response")).first().await.unwrap();
        response.wait_until().displayed().await.unwrap();
        println!("reponse: {}", response.text().await.unwrap());
        let response_text = response.text().await.unwrap();
        let score = response_text
            .lines()
            .find(|line| line.contains("\"score\":"))
            .and_then(|line| {
                let start_index = line.find(':')?;
                let end_index = line.find(',')?;
                line.get(start_index + 1..end_index)
            })
            .and_then(|score_str| score_str.trim().parse::<f32>().ok());
        score
    }

    #[tokio::test]
    async fn recaptcha() {
        let (driver, mut handle) = chrome().await.unwrap();
        let score = get_score(&driver).await;
        driver.quit().await.unwrap();
        handle.kill().unwrap();
        assert!(score.unwrap_or(0.0) >= 0.7);
    }
}
