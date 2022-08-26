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
        "HBAN", "DB", "MRVL", "WBA", "CDNA", "RSX", "KNX", "KBH", "DISCK", "PGR", "XLE", "MO",
        "MDT", "RIG", "BCS", "CDEV", "EOG", "XLP", "HPE", "TXN", "OAS", "OKE", "CVS", "SVXY",
        "HES", "GDDY", "BX", "NLSN", "TS", "BOX", "OXY", "CVE", "SYF", "IAU", "MTG", "LUV", "FTI",
        "AA", "ABEV", "VTI", "EQR", "EWG", "ETSY", "TAL", "UNH", "GES", "AMZN", "NFLX", "ENTG",
        "CSX",
    ];
    let result = fmp.batch_companies(tickers).await;

    let companies = result.unwrap();
    assert_eq!(companies.len(), 50);
}
