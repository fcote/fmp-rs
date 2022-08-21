use serde::de::DeserializeOwned;
use reqwest::{Response, StatusCode};

pub mod period;
pub mod stock;
pub mod analyst_estimate;
pub mod company;
pub mod financial;
pub mod quote;
pub mod historical_price;
pub mod earning;
pub mod news;
pub mod forex;

pub struct Client {
    pub base: String,
    pub api_key: String
}

impl Client {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        Client {
            base: endpoint.to_string(),
            api_key: api_key.to_string()
        }
    }
}

async fn decode_content<T>(response: Response) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    let content = response.json::<T>().await;
    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn request<T>(endpoint: String) -> Result<T, StatusCode>
where
    T: DeserializeOwned
{
    let response = reqwest::get(endpoint).await;
    match response {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
            decode_content(r).await
        }
        Err(e) => {
            return if e.is_status() {
                Err(e.status().unwrap())
            } else {
                println!("{:?}", e);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }
}