use crate::client::Client;
use crate::errors::*;
use crate::model;
use rust_decimal::Decimal;

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

    pub fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<model::OrderInfo>>
    where
        S: Into<String>,
    {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders?market={}", symbol.into());
        let data = self.client.get_signed(endpoint.into(), payload, None)?;

        let open_orders: model::ResultData<Vec<model::OrderInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(open_orders.result)
    }

    pub fn get_order_history<S>(
        &self,
        symbol: S,
        side: Option<String>,
        order_type: String,
        start_time: i64,
        end_time: i64,
    ) -> Result<model::ResultData<Vec<model::OrderInfo>>>
    where
        S: Into<String>,
    {
        let mut payload =
            json!({ "orderType": order_type, "start_time": start_time, "end_time": end_time });
        if let Some(s) = side {
            payload = json!({ "side": s, "orderType": order_type, "start_time": start_time, "end_time": end_time });
        }
        let endpoint = format!("/orders/history?market={}", symbol.into());
        let data = self
            .client
            .get_signed(endpoint.into(), payload.to_string(), None)?;
        let history_orders: model::ResultData<Vec<model::OrderInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(history_orders)
    }

    pub fn get_open_trigger_orders<S>(
        &self,
        symbol: S,
        option_type: Option<String>,
    ) -> Result<Vec<model::TriggerOrderInfo>>
    where
        S: Into<String>,
    {
        let mut payload: String = format!("{}", "{}");
        if let Some(t) = option_type {
            payload = json!({ "type": t }).to_string();
        }
        let endpoint = format!("/conditional_orders?market={}", symbol.into());
        let data = self
            .client
            .get_signed(endpoint.into(), payload.to_string(), None)?;
        let trigger_orders: model::ResultData<Vec<model::TriggerOrderInfo>> =
            serde_json::from_str(data.as_str())?;
        Ok(trigger_orders.result)
    }

    pub fn place_order<S>(
        &self,
        symbol: S,
        side: String,
        price: Decimal,
        option_type: String,
        size: Decimal,
        reduce_only: bool,
        ioc: bool,
        post_only: bool,
        client_id: Option<String>,
        reject_on_price_band: bool,
    ) -> Result<model::OrderInfo>
    where
        S: Into<String>,
    {
        let payload = json!({
          "market": symbol.into(),
          "side": side,
          "price": price,
          "type": option_type,
          "size": size,
          "reduceOnly": reduce_only,
          "ioc": ioc,
          "postOnly": post_only,
          "clientId": client_id,
          "rejectOnPriceBand": reject_on_price_band
        });
        let data = self
            .client
            .post_signed("/orders".into(), payload.to_string(), None)?;
        let order: model::ResultData<model::OrderInfo> = serde_json::from_str(data.as_str())?;
        Ok(order.result)
    }

    pub fn modify_order(
        &self,
        order_id: i64,
        price: Decimal,
        size: Decimal,
        client_id: Option<String>,
    ) -> Result<model::OrderInfo> {
        let mut payload = json!({
          "price": price,
          "size": size,
        });
        if let Some(c) = client_id {
            payload = json!({
              "price": price,
              "size": size,
              "clientId": c,
            });
        }
        let endpoint = format!("/orders/{}/modify", order_id);
        let data = self
            .client
            .post_signed(endpoint.into(), payload.to_string(), None)?;
        let order: model::ResultData<model::OrderInfo> = serde_json::from_str(data.as_str())?;
        Ok(order.result)
    }

    pub fn modify_order_by_client_id(
        &self,
        client_id: String,
        price: Decimal,
        size: Decimal,
    ) -> Result<model::OrderInfo> {
        let payload = json!({
          "price": price,
          "size": size,
        });
        let endpoint = format!("/orders/by_client_id/{}/modify", client_id);
        let data = self
            .client
            .post_signed(endpoint.into(), payload.to_string(), None)?;
        let order: model::ResultData<model::OrderInfo> = serde_json::from_str(data.as_str())?;
        Ok(order.result)
    }

    pub fn get_order_status(&self, order_id: i64) -> Result<model::OrderInfo> {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders/{}", order_id);
        let data = self.client.get_signed(endpoint.into(), payload, None)?;
        let order: model::ResultData<model::OrderInfo> = serde_json::from_str(data.as_str())?;
        Ok(order.result)
    }

    pub fn get_order_status_by_client_id(&self, client_id: String) -> Result<model::OrderInfo> {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders/by_client_id/{}", client_id);
        let data = self.client.get_signed(endpoint.into(), payload, None)?;
        let order: model::ResultData<model::OrderInfo> = serde_json::from_str(data.as_str())?;
        Ok(order.result)
    }

    pub fn cancel_order(&self, order_id: i64) -> Result<bool> {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders/{}", order_id);
        let data = self.client.delete_signed(endpoint.into(), payload, None)?;
        let result: model::ResultData<String> = serde_json::from_str(data.as_str())?;
        Ok(result.success)
    }

    pub fn cancel_order_by_client_id(&self, client_id: String) -> Result<bool> {
        let payload: String = format!("{}", "{}");
        let endpoint = format!("/orders/by_client_id/{}", client_id);
        let data = self.client.delete_signed(endpoint.into(), payload, None)?;
        println!("{:?}", data);
        let result: model::ResultData<String> = serde_json::from_str(data.as_str())?;
        Ok(result.success)
    }

    pub fn cancel_all_orders<S>(
        &self,
        symbol: S,
        side: Option<String>,
        conditional_orders_only: bool,
        limit_orders_only: bool,
    ) -> Result<bool>
    where
        S: Into<String> + Copy,
    {
        let mut payload = json!({
            "market": &symbol.into(),
            "conditionalOrdersOnly": conditional_orders_only,
            "limitOrdersOnly": limit_orders_only,
        });
        if let Some(s) = side {
            payload = json!({
                "market": &symbol.into(),
                "side": s,
                "conditionalOrdersOnly": conditional_orders_only,
                "limitOrdersOnly": limit_orders_only,
            });
        }
        let data = self
            .client
            .delete_signed("/orders".into(), payload.to_string(), None)?;
        let result: model::ResultData<String> = serde_json::from_str(data.as_str())?;
        Ok(result.success)
    }
}
