use serde::Deserialize;

/// A type for records in a CSV table with the following columns...
/// Account Number,Investment Name,Symbol,Shares,Share Price,Total Value,
#[derive(Deserialize, Debug)]
pub struct Account {
    #[serde(alias = "Account Number")]
    pub account_number: String,

    #[serde(alias = "Investment Name")]
    pub investment_name: String,

    #[serde(alias = "Symbol")]
    pub symbol: String,

    #[serde(alias = "Shares")]
    pub shares: f32,

    #[serde(alias = "Share Price")]
    pub share_price: f32,

    #[serde(alias = "Total Value")]
    pub total_value: f32,
}

/// A type for records in a CSV table with the following columns...
/// Account Number,Trade Date,Settlement Date,Transaction Type,Transaction Description,
/// Investment Name,Symbol,Shares,Share Price,Principal Amount,Commissions and Fees,Net Amount,
/// Accrued Interest,Account Type,
#[derive(Deserialize, Debug)]
pub struct Transaction {
    #[serde(alias = "Account Number")]
    pub account_number: String,

    #[serde(alias = "Trade Date")]
    pub trade_date: String,

    #[serde(alias = "Settlement Date")]
    pub settlement_date: String,

    #[serde(alias = "Transaction Type")]
    pub transaction_type: String,

    #[serde(alias = "Transaction Description")]
    pub transaction_description: String,

    #[serde(alias = "Investment Name")]
    pub investment_name: String,

    #[serde(alias = "Symbol")]
    pub symbol: String,

    #[serde(alias = "Shares")]
    pub shares: f32,

    #[serde(alias = "Share Price")]
    pub share_price: f32,

    #[serde(alias = "Principal Amount")]
    pub principal_amount: f32,

    #[serde(alias = "Commissions and Fees")]
    pub commissions_and_fees: f32,

    #[serde(alias = "Net Amount")]
    pub net_amount: f32,

    #[serde(alias = "Accrued Interest")]
    pub accrued_interest: f32,

    #[serde(alias = "Account Type")]
    pub account_type: String,
}
