# Financial Modeling Prep Web API | Rust SDK 🦀

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
  
  let result = fmp.income_statement("AAPL", FMPPeriod::YEAR).await;
  let statements = result.unwrap();
  println!("{:?}", statements);
}
```
