mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_news() {
    let fmp = get_client();

    let result = fmp.news("AAPL", 100).await;

    let quotes = result.unwrap();
    assert_eq!(quotes.len(), 100);
    let quote = quotes.first();
    assert_ne!(quote, None)
}

#[tokio::test]
async fn test_press_releases() {
    let fmp = get_client();

    let result = fmp.press_releases("AAPL", 100).await;

    let quotes = result.unwrap();
    assert_eq!(quotes.len(), 100);
    let quote = quotes.first();
    assert_ne!(quote, None)
}
