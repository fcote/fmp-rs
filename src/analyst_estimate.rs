use serde::{Serialize, Deserialize};
use crate::{Client, request, StatusCode};
use crate::period::FMPPeriod;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPAnalystEstimates {
    pub symbol: String,
    pub date: String,
    pub estimated_revenue_low: f64,
    pub estimated_revenue_high: f64,
    pub estimated_revenue_avg: f64,
    pub estimated_ebitda_low: f64,
    pub estimated_ebitda_high: f64,
    pub estimated_ebitda_avg: f64,
    pub estimated_ebit_low: f64,
    pub estimated_ebit_high: f64,
    pub estimated_ebit_avg: f64,
    pub estimated_net_income_low: f64,
    pub estimated_net_income_high: f64,
    pub estimated_net_income_avg: f64,
    pub estimated_sga_expense_low: f64,
    pub estimated_sga_expense_high: f64,
    pub estimated_sga_expense_avg: f64,
    pub estimated_eps_avg: f64,
    pub estimated_eps_high: f64,
    pub estimated_eps_low: f64,
    pub number_analyst_estimated_revenue: f64,
    pub number_analysts_estimated_eps: f64,
}

impl Client {
    pub async fn analyst_estimates(&self, ticker: &str, period: FMPPeriod) -> Result<Vec<FMPAnalystEstimates>, StatusCode> {
        request(format!(
            "{}/v3/analyst-estimates/{}?period={}&apikey={}",
            self.base,
            ticker,
            period.value(),
            self.api_key,
        )).await
    }
}
