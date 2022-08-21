mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_historical_prices() {
    let fmp = get_client();

    let result = fmp.historical_prices("AAPL").await;
    let prices = result.unwrap();
    let price = prices.first();

    assert_ne!(price, None)
}