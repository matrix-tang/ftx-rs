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

    pub fn ticker_all(&self) -> Result<Vec<model::TickerData>> {
        let data = self.client.get("/markets".into(), "".into())?;
        let ticker: model::Ticker = serde_json::from_str(data.as_str())?;
        Ok(ticker.result)
    }

    pub fn ticker<S>(&self, symbol: S) -> Result<model::TickerData>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/markets/{}", symbol.into());
        let data = self.client.get(endpoint, "".into())?;
        let ticker: model::TickerOne = serde_json::from_str(data.as_str())?;
        Ok(ticker.result)
    }

    // depth -> max 100, default 20
    pub fn orderbook<S, D>(&self, symbol: S, depth: D) -> Result<model::Depth>
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
        let depth: model::OrderBook = serde_json::from_str(data.as_str())?;
        Ok(depth.result)
    }

    pub fn trades<S>(&self, symbol: S) -> Result<Vec<model::Trade>>
    where
        S: Into<String>,
    {
        let endpoint: String = format!("/markets/{}/trades", symbol.into());
        let data = self.client.get(endpoint, "".into())?;
        let trades: model::Trades = serde_json::from_str(data.as_str())?;
        Ok(trades.result)
    }

    pub fn candles<S, R, ST, ET>(
        &self,
        symbol: S,
        resolution: R,
        start_time: ST,
        end_time: ET,
    ) -> Result<Vec<model::Candle>>
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
        let candles: model::Candles = serde_json::from_str(data.as_str())?;
        Ok(candles.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ticker() {
        let client = MarketsClient::new();
        let ticker = client.ticker("BTC-PERP");
        println!("{:#?}", ticker);
    }
}
