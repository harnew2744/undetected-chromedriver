#[cfg(test)]
mod tests {
    use undetected_chromedriver::chrome;

    #[tokio::test]
    async fn test_chrome() {
        let driver = chrome(None).await.unwrap();
        assert!(driver.title().await.is_ok());
        driver.quit().await.unwrap();
    }
}
