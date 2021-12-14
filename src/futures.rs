use crate::client::Client;
use crate::errors::*;
use crate::model;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FuturesClient {
    client: Client,
}

impl FuturesClient {
    pub fn new() -> Self {
        FuturesClient {
            client: Client::new(None, None),
        }
    }

    pub fn get_futures(&self) -> Result<Vec<model::Future>> {
        let data = self.client.get("/futures".into(), "".into())?;
        let futures: model::Futures = serde_json::from_str(data.as_str())?;
        Ok(futures.result)
    }

    pub fn get_future<S>(&self, symbol: S) -> Result<model::Future>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/futures/{}", symbol.into());
        let data = self.client.get(endpoint.into(), "".into())?;
        let future: model::FutureOne = serde_json::from_str(data.as_str())?;
        Ok(future.result)
    }

    pub fn get_stats<S>(&self, symbol: S) -> Result<model::StatsInfo>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/futures/{}/stats", symbol.into());
        let data = self.client.get(endpoint.into(), "".into())?;
        let stats: model::Stats = serde_json::from_str(data.as_str())?;
        Ok(stats.result)
    }

    pub fn get_funding_rates(&self) -> Result<Vec<model::FundingRate>> {
        let data = self.client.get("/funding_rates".into(), "".into())?;
        let funding_rates: model::FundingRates = serde_json::from_str(data.as_str())?;
        Ok(funding_rates.result)
    }

    // index_name -> ALT/MID/SHIT/EXCH/DRAGON
    pub fn get_indexes_weights<I>(&self, index_name: I) -> Result<HashMap<String, Decimal>>
    where
        I: Into<String>,
    {
        let endpoint: String = format!("/indexes/{}/weights", index_name.into());
        let data = self.client.get(endpoint.into(), "".into())?;
        let indexes_weights: model::IndexesWeights = serde_json::from_str(data.as_str())?;
        Ok(indexes_weights.result)
    }

    pub fn get_expired_futures(&self) -> Result<Vec<model::ExpiredFuture>> {
        let data = self.client.get("/expired_futures".into(), "".into())?;
        let expired_futures: model::ExpiredFutures = serde_json::from_str(data.as_str())?;
        Ok(expired_futures.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        let f = FuturesClient::new();
        let w = f.get_expired_futures();
        println!("{:#?}", w);
    }

    #[test]
    fn test_futures() {
        let f = FuturesClient::new();
        let all = f.get_futures();
        println!("{:#?}", all);
    }

    #[test]
    fn test_get_future() {
        let f = FuturesClient::new();
        let one = f.get_future("ZRX-PERP");
        println!("{:#?}", one);
    }

    #[test]
    fn test_get_stats() {
        let f = FuturesClient::new();
        let stats = f.get_stats("BTC-PERP");
        println!("{:#?}", stats);
    }

    #[test]
    fn test_get_funding_rates() {
        let f = FuturesClient::new();
        let rates = f.get_funding_rates();
        println!("{:#?}", rates);
    }
}
