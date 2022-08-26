use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPHistoricalPrice {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub adj_close: f64,
    pub volume: Option<f64>,
    pub unadjusted_volume: Option<f64>,
    pub change: f64,
    pub change_percent: f64,
    pub vwap: Option<f64>,
    pub label: String,
    pub change_over_time: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPHistoricalPriceResponse {
    pub symbol: String,
    pub historical: Vec<FMPHistoricalPrice>,
}

impl Client {
    pub async fn historical_prices(
        &self,
        ticker: &str,
    ) -> Result<Vec<FMPHistoricalPrice>, StatusCode> {
        let response = request::<FMPHistoricalPriceResponse>(format!(
            "{}/v3/historical-price-full/{}?from=1980-01-01&apikey={}",
            self.base, ticker, self.api_key,
        ))
        .await?;
        Ok(response.historical)
    }
}
