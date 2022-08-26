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

    let tickers = vec![
        "BTU", "JBGS", "IPG", "DE", "KHC", "COG", "IBM", "NYT", "DBJP", "AAL", "OLN", "JCI",
        "VICI", "VST", "NWL", "EPD", "KMB", "AEP", "IT", "CAT", "AU", "XRAY", "KKR", "IEUR", "LQD",
        "CMC", "XBI", "TMUS", "EWW", "NRG", "CPB", "ATHM", "WRB", "MOMO", "TGT", "TV", "GLW",
        "MAS", "DOV", "HRL", "TECK", "AES", "PPL", "IXUS", "SEE", "NLY", "FRC", "EWT", "STX",
        "ADBE",
    ];
    let result = fmp.batch_companies(tickers).await;

    let companies = result.unwrap();
    assert_eq!(companies.len(), 50);
}
