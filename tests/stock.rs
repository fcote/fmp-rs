mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_batch_stocks() {
    let fmp = get_client();

    let result = fmp.batch_stocks(vec!["AAPL", "NVDA"]).await;
    let stocks = result.unwrap();

    assert_eq!(stocks.len(), 2);
    let aapl = stocks.get(0).unwrap();
    assert_eq!(aapl.symbol, "AAPL");
    let nvda = stocks.get(1).unwrap();
    assert_eq!(nvda.symbol, "NVDA");
}

#[tokio::test]
async fn test_stock() {
    let fmp = get_client();

    let result = fmp.stock("AAPL").await;
    let stock = result.unwrap().unwrap();

    assert_eq!(stock.symbol, "AAPL");
}

#[tokio::test]
async fn test_stock_search() {
    let fmp = get_client();

    let result = fmp.stock_search("AAPL").await;
    let stocks = result.unwrap();
    let stock = stocks.first().unwrap();

    assert_eq!(stock.symbol, "AAPL");
}

#[tokio::test]
async fn test_stock_list() {
    let fmp = get_client();

    let result = fmp.stock_list().await;
    let stocks = result.unwrap();

    assert_ne!(stocks.first(), None);
}