use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPQuote {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub changes_percentage: f64,
    pub change: f64,
    pub day_low: f64,
    pub day_high: f64,
    pub year_high: f64,
    pub year_low: f64,
    pub market_cap: f64,
    pub price_avg50: f64,
    pub price_avg200: f64,
    pub volume: f64,
    pub avg_volume: f64,
    pub exchange: String,
    pub open: f64,
    pub previous_close: f64,
    pub eps: f64,
    pub pe: f64,
    pub earnings_announcement: String,
    pub shares_outstanding: f64,
    pub timestamp: f64,
}

impl Client {
    pub async fn batch_quotes(&self, tickers: Vec<&str>) -> Result<Vec<FMPQuote>, StatusCode> {
        let ticker = tickers.join(",");
        request(format!(
            "{}/v3/quote/{}?apikey={}",
            self.base, ticker, self.api_key,
        ))
        .await
    }

    pub async fn quote(&self, ticker: &str) -> Result<Option<FMPQuote>, StatusCode> {
        let quotes = self.batch_quotes(vec![ticker]).await?;
        Ok(quotes.into_iter().next())
    }
}
