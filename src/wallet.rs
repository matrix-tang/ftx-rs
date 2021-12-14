use crate::client::Client;
use crate::errors::*;
use crate::model;
use std::collections::HashMap;

#[derive(Clone)]
pub struct WalletClient {
    client: Client,
}

impl WalletClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        WalletClient {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_coins(&self) -> Result<Vec<model::CoinInfo>> {
        let data = self.client.get("/wallet/coins".into(), "".into())?;
        let coins: model::Coins = serde_json::from_str(data.as_str())?;
        Ok(coins.result)
    }

    pub fn get_balances(&self) -> Result<Vec<model::BalanceInfo>> {
        let payload: String = format!("{}", "{}");
        let data = self
            .client
            .get_signed("/wallet/balances".into(), payload, None)?;
        let balances: model::ResultData<Vec<model::BalanceInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(balances.result)
    }

    pub fn get_all_balances(&self) -> Result<HashMap<String, Vec<model::BalanceInfo>>> {
        let payload: String = format!("{}", "{}");
        let data = self
            .client
            .get_signed("/wallet/all_balances".into(), payload, None)?;
        let all_balances: model::ResultData<HashMap<String, Vec<model::BalanceInfo>>> =
            serde_json::from_str(data.as_str())?;
        Ok(all_balances.result)
    }

    pub fn get_deposit_address(
        &self,
        coin: String,
        method: Option<String>,
    ) -> Result<model::Address> {
        let mut endpoint = format!("/wallet/deposit_address/{}", &coin);
        if let Some(m) = method {
            endpoint = format!("/wallet/deposit_address/{}?method={}", coin, m);
        }
        let payload: String = format!("{}", "{}");
        let data = self
            .client
            .get_signed(endpoint.into(), payload.to_string(), None)?;
        let address: model::ResultData<model::Address> = serde_json::from_str(data.as_str())?;
        Ok(address.result)
    }

    pub fn get_deposits(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<model::Deposit>> {
        let mut payload: String = format!("{}", "{}");
        if start_time != None && end_time != None {
            payload = json!({ "start_time": start_time.unwrap(), "end_time": end_time.unwrap() })
                .to_string();
        }
        let data = self
            .client
            .get_signed("/wallet/deposits".into(), payload.to_string(), None)?;
        let deposits: model::ResultData<Vec<model::Deposit>> = serde_json::from_str(data.as_str())?;
        Ok(deposits.result)
    }

    pub fn get_withdrawals(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<model::Withdrawal>> {
        let mut payload: String = format!("{}", "{}");
        if start_time != None && end_time != None {
            payload = json!({ "start_time": start_time.unwrap(), "end_time": end_time.unwrap() })
                .to_string();
        }
        let data =
            self.client
                .get_signed("/wallet/withdrawals".into(), payload.to_string(), None)?;
        let withdrawals: model::ResultData<Vec<model::Withdrawal>> =
            serde_json::from_str(data.as_str())?;
        Ok(withdrawals.result)
    }

    pub fn get_airdrops(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<model::Airdrops>> {
        let mut payload: String = format!("{}", "{}");
        if start_time != None && end_time != None {
            payload = json!({ "start_time": start_time.unwrap(), "end_time": end_time.unwrap() })
                .to_string();
        }
        let data = self
            .client
            .get_signed("/wallet/airdrops".into(), payload.to_string(), None)?;
        let airdrops: model::ResultData<Vec<model::Airdrops>> =
            serde_json::from_str(data.as_str())?;
        Ok(airdrops.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_balances() {
        let client = WalletClient::new(None, None);
        let balances = client.get_balances();
        println!("{:#?}", balances);
    }
}
