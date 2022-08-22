# Financial Modeling Prep Web API | Rust SDK 🦀

fmp-rs is a wrapper for the Financial Modeling Prep Web API. 

This is only a partial implementation, feel free to submit a pull request for new endpoints. Also it currently only supports async, a pull request for a blocking client would be welcome.

To learn how to use fmp-rs, please refer to the usage section below.

## Cargo

```toml
[dependencies]

fmp = "0.1"
```

## Usage

```rust
use fmp::Client;
use fmp::period::FMPPeriod;

#[tokio::main]
async fn main() {
  let client = Client::new(
      "https://financialmodelingprep.com/api",
      "<FMP_API_KEY>",
  )
  
  let result = fmp.income_statements("AAPL", FMPPeriod::YEAR).await;
  let statements = result.unwrap();
  println!("{:?}", statements);
}
```

## Available endpoints

- Stock search | `/v3/search?query=AA`
- Stock list | `/v3/stock/list`
- Stock quote | `/v3/quote/AAPL`
- Historical price | `/v3/historical-price-full/AAPL`
- Analyst estimates | `/v3/analyst-estimates/AAPL`
- Company profile | `/v3/profile/AAPL`
- Earning
  - Calendar | `/v3/historical/earning_calendar/AAPL?limit=80`
  - Call transcript list | `/v4/earning_call_transcript?symbol=AAPL`
  - Call transcript | `/v3/earning_call_transcript/AAPL?quarter=3&year=2020`
- Financial statements
  - Income | `/v3/income-statement/AAPL?period=quarter`
  - Balance sheet | `/v3/balance-sheet-statement/AAPL?period=quarter`
  - Cash flow | `/v3/cash-flow-statement/AAPL?period=quarter`
- Forex | `/v3/quotes/forex`
- News | `/v3/stock_news?tickers=AAPL&limit=50`

## Contributing
If you find any problems or have suggestions about this crate, please submit an issue. Moreover, any pull request, code review and feedback are welcome.