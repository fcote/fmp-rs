use crate::period::FMPPeriod;
use crate::{request, Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPIncomeStatement {
    pub date: String,
    pub symbol: String,
    pub reported_currency: String,
    pub cik: String,
    pub filling_date: String,
    pub accepted_date: String,
    pub calendar_year: String,
    pub period: String,
    pub revenue: f64,
    pub cost_of_revenue: f64,
    pub gross_profit: f64,
    pub gross_profit_ratio: f64,
    pub research_and_development_expenses: f64,
    pub general_and_administrative_expenses: f64,
    pub selling_and_marketing_expenses: f64,
    pub selling_general_and_administrative_expenses: f64,
    pub other_expenses: f64,
    pub operating_expenses: f64,
    pub cost_and_expenses: f64,
    pub interest_income: f64,
    pub interest_expense: f64,
    pub depreciation_and_amortization: f64,
    pub ebitda: f64,
    pub ebitdaratio: f64,
    pub operating_income: f64,
    pub operating_income_ratio: f64,
    pub total_other_income_expenses_net: f64,
    pub income_before_tax: f64,
    pub income_before_tax_ratio: f64,
    pub income_tax_expense: f64,
    pub net_income: f64,
    pub net_income_ratio: f64,
    pub eps: f64,
    pub epsdiluted: f64,
    pub weighted_average_shs_out: f64,
    pub weighted_average_shs_out_dil: f64,
    pub link: String,
    pub final_link: String,
}

impl Client {
    pub async fn income_statements(
        &self,
        ticker: &str,
        period: FMPPeriod,
    ) -> Result<Vec<FMPIncomeStatement>, StatusCode> {
        request(format!(
            "{}/v3/income-statement/{}?period={}&apikey={}",
            self.base,
            ticker,
            period.value(),
            self.api_key,
        ))
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPBalanceSheetStatement {
    pub date: String,
    pub symbol: String,
    pub reported_currency: String,
    pub cik: String,
    pub filling_date: String,
    pub accepted_date: String,
    pub calendar_year: String,
    pub period: String,
    pub cash_and_cash_equivalents: f64,
    pub short_term_investments: f64,
    pub cash_and_short_term_investments: f64,
    pub net_receivables: f64,
    pub inventory: f64,
    pub other_current_assets: f64,
    pub total_current_assets: f64,
    pub property_plant_equipment_net: f64,
    pub goodwill: f64,
    pub intangible_assets: f64,
    pub goodwill_and_intangible_assets: f64,
    pub long_term_investments: f64,
    pub tax_assets: f64,
    pub other_non_current_assets: f64,
    pub total_non_current_assets: f64,
    pub other_assets: f64,
    pub total_assets: f64,
    pub account_payables: f64,
    pub short_term_debt: f64,
    pub tax_payables: f64,
    pub deferred_revenue: f64,
    pub other_current_liabilities: f64,
    pub total_current_liabilities: f64,
    pub long_term_debt: f64,
    pub deferred_revenue_non_current: f64,
    pub deferred_tax_liabilities_non_current: f64,
    pub other_non_current_liabilities: f64,
    pub total_non_current_liabilities: f64,
    pub other_liabilities: f64,
    pub capital_lease_obligations: f64,
    pub total_liabilities: f64,
    pub preferred_stock: f64,
    pub common_stock: f64,
    pub retained_earnings: f64,
    pub accumulated_other_comprehensive_income_loss: f64,
    pub othertotal_stockholders_equity: f64,
    pub total_stockholders_equity: f64,
    pub total_liabilities_and_stockholders_equity: f64,
    pub minority_interest: f64,
    pub total_equity: f64,
    pub total_liabilities_and_total_equity: f64,
    pub total_investments: f64,
    pub total_debt: f64,
    pub net_debt: f64,
    pub link: String,
    pub final_link: String,
}

impl Client {
    pub async fn balance_sheet_statements(
        &self,
        ticker: &str,
        period: FMPPeriod,
    ) -> Result<Vec<FMPBalanceSheetStatement>, StatusCode> {
        request(format!(
            "{}/v3/balance-sheet-statement/{}?period={}&apikey={}",
            self.base,
            ticker,
            period.value(),
            self.api_key,
        ))
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FMPCashFlowStatement {
    pub date: String,
    pub symbol: String,
    pub reported_currency: String,
    pub cik: String,
    pub filling_date: String,
    pub accepted_date: String,
    pub calendar_year: String,
    pub period: String,
    pub net_income: f64,
    pub depreciation_and_amortization: f64,
    pub deferred_income_tax: f64,
    pub stock_based_compensation: f64,
    pub change_in_working_capital: f64,
    pub accounts_receivables: f64,
    pub inventory: f64,
    pub accounts_payables: f64,
    pub other_working_capital: f64,
    pub other_non_cash_items: f64,
    pub net_cash_provided_by_operating_activities: f64,
    pub investments_in_property_plant_and_equipment: f64,
    pub acquisitions_net: f64,
    pub purchases_of_investments: f64,
    pub sales_maturities_of_investments: f64,
    pub other_investing_activites: f64,
    pub net_cash_used_for_investing_activites: f64,
    pub debt_repayment: f64,
    pub common_stock_issued: f64,
    pub common_stock_repurchased: f64,
    pub dividends_paid: f64,
    pub other_financing_activites: f64,
    pub net_cash_used_provided_by_financing_activities: f64,
    pub effect_of_forex_changes_on_cash: f64,
    pub net_change_in_cash: f64,
    pub cash_at_end_of_period: f64,
    pub cash_at_beginning_of_period: f64,
    pub operating_cash_flow: f64,
    pub capital_expenditure: f64,
    pub free_cash_flow: f64,
    pub link: String,
    pub final_link: String,
}

impl Client {
    pub async fn cash_flow_statements(
        &self,
        ticker: &str,
        period: FMPPeriod,
    ) -> Result<Vec<FMPCashFlowStatement>, StatusCode> {
        request(format!(
            "{}/v3/cash-flow-statement/{}?period={}&apikey={}",
            self.base,
            ticker,
            period.value(),
            self.api_key,
        ))
        .await
    }
}
