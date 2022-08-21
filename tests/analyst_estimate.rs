mod helpers;
use helpers::client::get_client;
use fmp::period::FMPPeriod;

#[tokio::test]
async fn test_analyst_estimates() {
    let fmp = get_client();

    let result = fmp.analyst_estimates(
        "AAPL",
        FMPPeriod::YEAR,
    ).await;

    let estimates = result.unwrap();
    let estimate = estimates.first().unwrap();
    assert_eq!(estimate.symbol, "AAPL")
}