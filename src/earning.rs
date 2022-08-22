use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPEarning {
    pub date: String,
    pub symbol: String,
    pub eps: f64,
    pub eps_estimated: f64,
    pub time: String,
    pub revenue: f64,
    pub revenue_estimated: f64,
    pub updated_from_date: Option<String>,
    pub fiscal_date_ending: String,
}

impl Client {
    pub async fn earnings(&self, ticker: &str) -> Result<Vec<FMPEarning>, StatusCode> {
        request(format!(
            "{}/v3/historical/earning_calendar/{}?apikey={}",
            self.base, ticker, self.api_key,
        ))
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPEarningCallTranscript {
    pub symbol: String,
    pub quarter: f64,
    pub year: f64,
    pub date: String,
    pub content: String,
}

impl Client {
    pub async fn earning_call_transcript(
        &self,
        ticker: &str,
        quarter: i8,
        year: i16,
    ) -> Result<Option<FMPEarningCallTranscript>, StatusCode> {
        let transcripts = request::<Vec<FMPEarningCallTranscript>>(format!(
            "{}/v3/earning_call_transcript/{}?quarter={}&year={}&apikey={}",
            self.base, ticker, quarter, year, self.api_key,
        ))
        .await?;
        Ok(transcripts.into_iter().next())
    }
}

pub type FMPPartialEarningCallTranscript = (f64, f64, String);

impl Client {
    pub async fn earning_call_transcripts(
        &self,
        ticker: &str,
    ) -> Result<Vec<FMPPartialEarningCallTranscript>, StatusCode> {
        request(format!(
            "{}/v4/earning_call_transcript?symbol={}&apikey={}",
            self.base, ticker, self.api_key,
        ))
        .await
    }
}
