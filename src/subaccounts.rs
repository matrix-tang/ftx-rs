use crate::client::Client;
use crate::errors::*;
use crate::model;

#[derive(Clone)]
pub struct SubAccountsClient {
    client: Client,
}

impl SubAccountsClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        SubAccountsClient {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_subaccounts(&self) -> Result<Vec<model::SubAccount>> {
        let payload: String = format!("{}", "{}");
        let data = self
            .client
            .get_signed("/subaccounts".into(), payload, None)?;
        let subaccounts: model::SubAccounts = serde_json::from_str(data.as_str())?;
        Ok(subaccounts.result)
    }

    pub fn create_subaccount(&self, nickname: String) -> Result<model::SubAccount> {
        let payload = json!({ "nickname": nickname });
        let data = self
            .client
            .post_signed("/subaccounts".into(), payload.to_string(), None)?;
        let subaccount: model::SubAccountOne = serde_json::from_str(data.as_str())?;
        Ok(subaccount.result.unwrap())
    }

    pub fn change_subaccount_name(&self, nickname: String, new_nickname: String) -> Result<bool> {
        let payload = json!({ "nickname": nickname, "newNickname": new_nickname });
        let data = self.client.post_signed(
            "/subaccounts/update_name".into(),
            payload.to_string(),
            None,
        )?;
        let r: model::SubAccountOne = serde_json::from_str(data.as_str())?;
        Ok(r.success)
    }

    pub fn delete_subaccount(&self, nickname: String) -> Result<bool> {
        let payload = json!({ "nickname": nickname });
        let data = self
            .client
            .delete_signed("/subaccounts".into(), payload.to_string(), None)?;
        let r: model::SubAccountOne = serde_json::from_str(data.as_str())?;
        Ok(r.success)
    }

    pub fn get_subaccount_balances(
        &self,
        nickname: String,
    ) -> Result<Vec<model::SubAccountBalance>> {
        let payload: String = format!("{}", "{}");

        let endpoint = format!("/subaccounts/{}/balances", nickname);
        let data = self.client.get_signed(endpoint.into(), payload, None)?;
        let subaccount_balances: model::SubAccountBalances = serde_json::from_str(data.as_str())?;
        Ok(subaccount_balances.result)
    }

    pub fn transfer_subaccounts(
        &self,
        coin: String,
        size: f64,
        source: String,
        destination: String,
    ) -> Result<model::Transfer> {
        let payload =
            json!({ "coin": coin, "size": size, "source": source, "destination": destination });

        // println!("{}", payload);
        let data =
            self.client
                .post_signed("/subaccounts/transfer".into(), payload.to_string(), None)?;
        // println!("{:#?}", data);
        let subacounts_transfer: model::SubaccountsTransfer = serde_json::from_str(data.as_str())?;
        Ok(subacounts_transfer.result)
    }
}
