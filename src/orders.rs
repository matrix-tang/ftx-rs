use crate::client::Client;
use crate::errors::*;
// use crate::model;

#[derive(Clone)]
pub struct OrdersClient {
    client: Client,
}

impl OrdersClient {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        OrdersClient {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn get_open_orders<S>(&self, symbol: S) -> Result<()>
    where
        S: Into<String>,
    {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders?market={}", symbol.into());
        let data = self.client.get_signed(endpoint.into(), payload, None)?;
        println!("{:?}", data);
        Ok(())
    }
}
