use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPNews {
    pub symbol: String,
    pub published_date: String,
    pub title: String,
    pub image: String,
    pub site: String,
    pub text: String,
    pub url: String,
}

impl Client {
    pub async fn news(&self, ticker: &str, limit: i32) -> Result<Vec<FMPNews>, StatusCode> {
        request(format!(
            "{}/v3/stock_news?tickers={}&limit={}&apikey={}",
            self.base, ticker, limit, self.api_key,
        ))
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPPressRelease {
    pub symbol: String,
    pub date: String,
    pub title: String,
    pub text: String,
}

impl Client {
    pub async fn press_releases(
        &self,
        ticker: &str,
        limit: i32,
    ) -> Result<Vec<FMPPressRelease>, StatusCode> {
        request(format!(
            "{}/v3/press-releases/{}?limit={}&apikey={}",
            self.base, ticker, limit, self.api_key,
        ))
        .await
    }
}
