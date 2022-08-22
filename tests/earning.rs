mod helpers;
use helpers::client::get_client;

#[tokio::test]
async fn test_earnings() {
    let fmp = get_client();

    let result = fmp.earnings("AAPL").await;
    let earnings = result.unwrap();
    let earning = earnings.first().unwrap();

    assert_eq!(earning.symbol, "AAPL")
}

#[tokio::test]
async fn test_earning_call_transcript() {
    let fmp = get_client();

    let result = fmp.earning_call_transcript("AAPL", 3, 2022).await;
    let transcript = result.unwrap().unwrap();

    assert_eq!(transcript.symbol, "AAPL")
}

#[tokio::test]
async fn test_earning_call_transcripts() {
    let fmp = get_client();

    let result = fmp.earning_call_transcripts("AAPL").await;
    let transcripts = result.unwrap();
    let transcript = transcripts.first();

    assert_ne!(transcript, None)
}
