use crate::{request, Client};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FMPCompany {
    pub symbol: String,
    pub price: f64,
    pub beta: f64,
    #[serde(rename = "volAvg")]
    pub vol_avg: i64,
    #[serde(rename = "mktCap")]
    pub mkt_cap: i64,
    #[serde(rename = "lastDiv")]
    pub last_div: f64,
    pub range: Option<String>,
    pub changes: f64,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    pub currency: Option<String>,
    pub cik: Option<String>,
    pub isin: Option<String>,
    pub cusip: Option<String>,
    pub exchange: Option<String>,
    #[serde(rename = "exchangeShortName")]
    pub exchange_short_name: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub ceo: Option<String>,
    pub sector: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "fullTimeEmployees")]
    pub full_time_employees: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    #[serde(rename = "dcfDiff")]
    pub dcf_diff: Option<f64>,
    pub dcf: Option<f64>,
    pub image: Option<String>,
    #[serde(rename = "ipoDate")]
    pub ipo_date: Option<String>,
    #[serde(rename = "defaultImage")]
    pub default_image: bool,
    #[serde(rename = "isEtf")]
    pub is_etf: bool,
    #[serde(rename = "isActivelyTrading")]
    pub is_actively_trading: bool,
    #[serde(rename = "isAdr")]
    pub is_adr: bool,
    #[serde(rename = "isFund")]
    pub is_fund: bool,
}

impl Client {
    pub async fn batch_companies(&self, tickers: Vec<&str>) -> Result<Vec<FMPCompany>, StatusCode> {
        request(format!(
            "{}/v3/profile/{}?apikey={}",
            self.base,
            tickers.join(","),
            self.api_key,
        ))
        .await
    }

    pub async fn company(&self, ticker: &str) -> Result<Option<FMPCompany>, StatusCode> {
        let companies = self.batch_companies(vec![ticker]).await?;
        Ok(companies.into_iter().next())
    }
}
