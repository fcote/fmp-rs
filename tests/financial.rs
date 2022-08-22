mod helpers;

use fmp::period::FMPPeriod;
use helpers::client::get_client;

#[tokio::test]
async fn test_income_statement() {
    let fmp = get_client();

    let result = fmp.income_statements("AAPL", FMPPeriod::YEAR).await;
    let statements = result.unwrap();
    let statement = statements.first().unwrap();
    assert_eq!(statement.symbol, "AAPL");
}


#[tokio::test]
async fn test_balance_sheet_statement() {
    let fmp = get_client();

    let result = fmp.balance_sheet_statements("AAPL", FMPPeriod::YEAR).await;
    let statements = result.unwrap();
    let statement = statements.first().unwrap();

    assert_eq!(statement.symbol, "AAPL");
}

#[tokio::test]
async fn test_cash_flow_statement() {
    let fmp = get_client();

    let result = fmp.cash_flow_statements("AAPL", FMPPeriod::YEAR).await;
    let statements = result.unwrap();
    let statement = statements.first().unwrap();

    assert_eq!(statement.symbol, "AAPL");
}