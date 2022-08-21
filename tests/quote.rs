mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_batch_quotes() {
    let fmp = get_client();

    let result = fmp.batch_quotes(vec!["AAPL", "NVDA"]).await;

    let quotes = result.unwrap();
    assert_eq!(quotes.len(), 2);
    let aapl = quotes.get(0).unwrap();
    assert_eq!(aapl.symbol, "AAPL");
    let nvda = quotes.get(1).unwrap();
    assert_eq!(nvda.symbol, "NVDA");
}

#[tokio::test]
async fn test_quotes() {
    let fmp = get_client();

    let result = fmp.quote("AAPL").await;
    let quote = result.unwrap().unwrap();

    assert_eq!(quote.symbol, "AAPL")
}