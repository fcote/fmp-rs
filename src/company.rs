use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use crate::{Client, request};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPCompany {
    pub symbol: String,
    pub price: f64,
    pub beta: f64,
    pub vol_avg: f64,
    pub mkt_cap: f64,
    pub last_div: f64,
    pub range: String,
    pub changes: f64,
    pub company_name: String,
    pub currency: String,
    pub cik: String,
    pub isin: String,
    pub cusip: String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub industry: String,
    pub website: String,
    pub description: String,
    pub ceo: String,
    pub sector: String,
    pub country: String,
    pub full_time_employees: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub dcf_diff: f64,
    pub dcf: f64,
    pub image: String,
    pub ipo_date: String,
    pub default_image: bool,
    pub is_etf: bool,
    pub is_actively_trading: bool,
    pub is_adr: bool,
    pub is_fund: bool,
}

impl Client {
    pub async fn batch_companies(&self, tickers: Vec<&str>) -> Result<Vec<FMPCompany>, StatusCode> {
        request(format!(
            "{}/v3/profile/{}?apikey={}",
            self.base,
            tickers.join(","),
            self.api_key,
        )).await
    }

    pub async fn company(&self, ticker: &str) -> Result<Option<FMPCompany>, StatusCode> {
        let companies = self.batch_companies(vec![ticker]).await?;
        Ok(companies.into_iter().next())
    }
}