use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPStock {
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
    pub async fn batch_stocks(&self, tickers: Vec<&str>) -> Result<Vec<FMPStock>, StatusCode> {
        request(format!(
            "{}/v3/quote/{}?apikey={}",
            self.base,
            tickers.join(","),
            self.api_key,
        ))
        .await
    }

    pub async fn stock(&self, ticker: &str) -> Result<Option<FMPStock>, StatusCode> {
        let stocks = self.batch_stocks(vec![ticker]).await?;
        Ok(stocks.into_iter().next())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPStockSearch {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    pub stock_exchange: String,
    pub exchange_short_name: String,
}

impl Client {
    pub async fn stock_search(&self, query: &str) -> Result<Vec<FMPStockSearch>, StatusCode> {
        request(format!(
            "{}/v3/search-ticker?query={}&apikey={}",
            self.base, query, self.api_key,
        ))
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPPartialStock {
    pub symbol: String,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub exchange: Option<String>,
    pub exchange_short_name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Client {
    pub async fn stock_list(&self) -> Result<Vec<FMPPartialStock>, StatusCode> {
        request(format!(
            "{}/v3/stock/list?apikey={}",
            self.base, self.api_key,
        ))
        .await
    }
}
