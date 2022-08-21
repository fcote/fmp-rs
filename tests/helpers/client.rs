use std::env;
use fmp::Client;

pub fn get_client() -> Client {
    let api_key = env::var("FMP_API_KEY").unwrap();
    Client::new(
        "https://financialmodelingprep.com/api",
        api_key.as_str(),
    )
}