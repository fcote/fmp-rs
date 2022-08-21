mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_company() {
    let fmp = get_client();

    let result = fmp.company("AAPL").await;

    let company = result.unwrap().unwrap();
    assert_eq!(company.symbol, "AAPL")
}

#[tokio::test]
async fn test_batch_companies() {
    let fmp = get_client();

    let tickers = vec!["AAPL", "NVDA"];
    let result = fmp.batch_companies(tickers).await;

    let companies = result.unwrap();
    assert_eq!(companies.len(), 2);
    let aapl = companies.get(0).unwrap();
    assert_eq!(aapl.symbol, "AAPL");
    let nvda = companies.get(1).unwrap();
    assert_eq!(nvda.symbol, "NVDA");
}