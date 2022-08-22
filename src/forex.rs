use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPForexQuote {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub changes_percentage: f64,
    pub change: f64,
    pub day_low: f64,
    pub day_high: f64,
    pub year_high: f64,
    pub year_low: f64,
    pub price_avg50: f64,
    pub price_avg200: f64,
    pub volume: f64,
    pub avg_volume: f64,
    pub exchange: String,
    pub open: f64,
    pub previous_close: f64,
    pub timestamp: f64,
}

impl Client {
    pub async fn forex_quotes(&self) -> Result<Vec<FMPForexQuote>, StatusCode> {
        request(format!(
            "{}/v3/quotes/forex?apikey={}",
            self.base, self.api_key,
        ))
        .await
    }
}
