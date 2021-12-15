use crate::client::Client;
use crate::errors::*;
use crate::model;

#[derive(Clone)]
pub struct MarketsClient {
    client: Client,
}

impl MarketsClient {
    pub fn new() -> Self {
        MarketsClient {
            client: Client::new(None, None),
        }
    }

    pub fn get_markets(&self) -> Result<Vec<model::MarketInfo>> {
        let data = self.client.get("/markets".into(), "".into())?;
        let markets: model::ResultData<Vec<model::MarketInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(markets.result)
    }

    pub fn get_market<S>(&self, symbol: S) -> Result<model::MarketInfo>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/markets/{}", symbol.into());
        let data = self.client.get(endpoint, "".into())?;
        let market: model::ResultData<model::MarketInfo> = serde_json::from_str(data.as_str())?;
        Ok(market.result)
    }

    // depth -> max 100, default 20
    pub fn get_orderbook<S, D>(&self, symbol: S, depth: D) -> Result<model::Depth>
    where
        S: Into<String>,
        D: Into<i64>,
    {
        let endpoint: String = format!(
            "/markets/{}/orderbook?depth={}",
            symbol.into(),
            depth.into()
        );
        let data = self.client.get(endpoint, "".into())?;
        let depth: model::ResultData<model::Depth> = serde_json::from_str(data.as_str())?;
        Ok(depth.result)
    }

    pub fn get_trades<S>(&self, symbol: S) -> Result<Vec<model::TradeInfo>>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/markets/{}/trades", symbol.into());
        let data = self.client.get(endpoint, "".into())?;
        let trades: model::ResultData<Vec<model::TradeInfo>> = serde_json::from_str(data.as_str())?;
        Ok(trades.result)
    }

    pub fn get_historical_prices<S, R, ST, ET>(
        &self,
        symbol: S,
        resolution: R,
        start_time: ST,
        end_time: ET,
    ) -> Result<Vec<model::CandleInfo>>
    where
        S: Into<String>,
        R: Into<i64>,
        ST: Into<i64>,
        ET: Into<i64>,
    {
        let endpoint: String = format!(
            "/markets/{}/candles?resolution={}&start_time={}&end_time={}",
            symbol.into(),
            resolution.into(),
            start_time.into(),
            end_time.into()
        );
        let data = self.client.get(endpoint, "".into())?;
        let candles: model::ResultData<Vec<model::CandleInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(candles.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_market() {
        let client = MarketsClient::new();
        let ticker = client.get_market("BTC-PERP");
        println!("{:#?}", ticker);
    }
}
