use rust_decimal::Decimal;
use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum NotificationEvent {
    Subscribed(SubscriptionMessage),
    Pong(PongMessage),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum DataEvent {
    TickerEvent(TickerData),
    TradesEvent(TradesData),
    OrderBookEvent(OrderBookData),
    FillsEvent(FillsData),
    OrdersEvent(OrdersData),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionMessage {
    #[serde(rename = "type")]
    pub option_type: String,
    pub channel: String,
    pub market: Option<String>,
    pub grouping: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PongMessage {
    #[serde(rename = "type")]
    pub option_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeInfo {
    pub id: i64,
    pub price: Decimal,
    pub size: Decimal,
    pub side: String,
    pub liquidation: bool,
    pub time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesData {
    pub channel: String,
    pub market: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub data: Vec<TradeInfo>,
}

#[derive(Debug, Deserialize)]
pub struct OrderBookInfo {
    pub time: String,
    pub checksum: i64,
    pub bids: Vec<Vec<Decimal>>,
    pub asks: Vec<Vec<Decimal>>,
    pub action: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookData {
    pub channel: String,
    pub market: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub data: OrderBookInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerInfo {
    pub bid: Decimal,
    pub ask: Decimal,
    pub bid_size: Decimal,
    pub ask_size: Decimal,
    pub last: Decimal,
    pub time: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    pub channel: String,
    pub market: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub data: TickerInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    pub fee: Decimal,
    pub fee_rate: Decimal,
    pub future: String,
    pub id: i64,
    pub liquidity: String,
    pub market: String,
    pub order_id: i64,
    pub trade_id: i64,
    pub price: Decimal,
    pub side: String,
    pub size: Decimal,
    pub time: String,
    #[serde(rename = "type")]
    pub option_type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillsData {
    pub channel: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub data: FillInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub id: i64,
    pub client_id: Option<String>,
    pub market: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub side: String,
    pub size: Decimal,
    pub price: Decimal,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub status: String,
    pub filled_size: Decimal,
    pub remaining_size: Decimal,
    pub avg_fill_price: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersData {
    pub channel: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub data: OrderInfo,
}
