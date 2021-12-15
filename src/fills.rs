use crate::client::Client;
use crate::errors::*;
use crate::model;

#[derive(Clone)]
pub struct FillsClient {
    client: Client,
}

impl FillsClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        FillsClient {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_fills<S>(
        &self,
        symbol: S,
        start_time: i64,
        end_time: i64,
    ) -> Result<Vec<model::FillInfo>>
    where
        S: Into<String> + Copy,
    {
        let payload = json!({
            "market": symbol.into(),
            "start_time": start_time,
            "end_time": end_time,
        });
        let endpoint = format!("/fills?market={}", symbol.into());
        let data = self
            .client
            .get_signed(endpoint.into(), payload.to_string(), None)?;
        let fills: model::ResultData<Vec<model::FillInfo>> = serde_json::from_str(data.as_str())?;
        Ok(fills.result)
    }
}
