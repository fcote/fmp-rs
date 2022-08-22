mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_forex_quotes() {
    let fmp = get_client();

    let result = fmp.forex_quotes().await;
    let quotes = result.unwrap();
    let quote = quotes.first();

    assert_ne!(quote, None)
}
