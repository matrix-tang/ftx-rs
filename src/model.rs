use rust_decimal::Decimal;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnotherOption {
    pub success: bool,
    pub(crate) result: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub name: String, //"BTC-0628",
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub quote_volume24h: Option<Decimal>,
    pub change1h: Option<Decimal>,
    pub change24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub high_leverage_fee_exempt: bool,
    pub min_provide_size: Option<Decimal>,
    #[serde(rename = "type")]
    pub market_type: String,
    pub underlying: Option<String>,
    pub enabled: bool,
    pub ask: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub last: Option<Decimal>,
    pub post_only: bool,
    pub price: Option<Decimal>,
    pub price_increment: Option<Decimal>,
    pub size_increment: Option<Decimal>,
    pub restricted: bool,
    pub volume_usd24h: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Depth {
    pub bids: Vec<Vec<Decimal>>,
    pub asks: Vec<Vec<Decimal>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeInfo {
    pub id: i64,
    pub price: Decimal,
    pub size: Decimal,
    pub side: String,
    pub liquidation: bool,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CandleInfo {
    pub start_time: String,
    pub time: Decimal,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubAccountInfo {
    pub nickname: String,
    pub special: bool,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountBalanceInfo {
    pub coin: String,
    pub total: Decimal,
    pub free: Decimal,
    pub available_without_borrow: Decimal,
    pub usd_value: Decimal,
    pub spot_borrow: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferInfo {
    pub id: i64,
    pub coin: String,
    pub size: f64,
    pub time: String,
    pub notes: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Future {
    pub name: String,
    pub underlying: String,
    pub description: String,
    #[serde(rename = "type")]
    pub market_type: String,
    pub expiry: Option<String>,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub price_increment: Option<Decimal>,
    pub size_increment: Option<Decimal>,
    pub last: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub index: Option<Decimal>,
    pub mark: Option<Decimal>,
    pub imf_factor: Option<Decimal>,
    pub lower_bound: Option<Decimal>,
    pub upper_bound: Option<Decimal>,
    pub underlying_description: String,
    pub expiry_description: String,
    pub move_start: Option<String>,
    pub margin_price: Option<Decimal>,
    pub position_limit_weight: Option<Decimal>,
    pub group: String,
    pub change1h: Option<Decimal>,
    pub change24h: Option<Decimal>,
    pub change_bod: Option<Decimal>,
    pub volume_usd24h: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub open_interest: Option<Decimal>,
    pub open_interest_usd: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Futures {
    pub success: bool,
    pub(crate) result: Vec<Future>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FutureOne {
    pub success: bool,
    pub(crate) result: Future,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatsInfo {
    pub volume: Option<Decimal>,
    pub next_funding_rate: Option<Decimal>,
    pub next_funding_time: String,
    pub expiration_price: Option<Decimal>,
    pub predicted_expiration_price: Option<Decimal>,
    pub strike_price: Option<Decimal>,
    pub open_interest: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub success: bool,
    pub(crate) result: StatsInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingRate {
    pub future: String,
    pub rate: Decimal,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FundingRates {
    pub success: bool,
    pub(crate) result: Vec<FundingRate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexesWeights {
    pub success: bool,
    pub(crate) result: HashMap<String, Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpiredFuture {
    pub name: String,
    pub underlying: String,
    pub description: String,
    #[serde(rename = "type")]
    pub marlet_type: String,
    pub expiry: String,
    pub perpetual: bool,
    pub expired: bool,
    pub enabled: bool,
    pub post_only: bool,
    pub price_increment: Option<Decimal>,
    pub size_increment: Option<Decimal>,
    pub last: Option<Decimal>,
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub index: Option<Decimal>,
    pub mark: Option<Decimal>,
    pub imf_factor: Option<Decimal>,
    pub lower_bound: Option<Decimal>,
    pub upper_bound: Option<Decimal>,
    pub underlying_description: String,
    pub expiry_description: String,
    pub move_start: Option<String>,
    pub margin_price: Option<Decimal>,
    pub position_limit_weight: Option<Decimal>,
    pub group: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpiredFutures {
    pub success: bool,
    pub(crate) result: Vec<ExpiredFuture>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub cost: Decimal,
    pub entry_price: Decimal,
    pub future: String,
    pub initial_margin_requirement: Decimal,
    pub long_order_size: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub net_size: Decimal,
    pub open_size: Decimal,
    pub realized_pnl: Decimal,
    pub short_order_size: Decimal,
    pub side: String,
    pub size: Decimal,
    pub unrealized_pnl: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Positions {
    pub success: bool,
    pub(crate) result: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_identifier: i64,
    pub username: String,
    pub collateral: Decimal,
    pub free_collateral: Decimal,
    pub total_account_value: Decimal,
    pub total_position_size: Decimal,
    pub initial_margin_requirement: Decimal,
    pub maintenance_margin_requirement: Decimal,
    pub margin_fraction: Option<Decimal>,
    pub open_margin_fraction: Option<Decimal>,
    pub liquidating: bool,
    pub backstop_provider: bool,
    pub positions: Vec<Position>,
    pub taker_fee: Decimal,
    pub maker_fee: Decimal,
    pub leverage: Decimal,
    pub position_limit: Option<Decimal>,
    pub position_limit_used: Option<Decimal>,
    pub use_ftt_collateral: bool,
    pub charge_interest_on_negative_usd: bool,
    pub spot_margin_enabled: bool,
    pub spot_lending_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub success: bool,
    pub(crate) result: AccountInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub id: String,
    pub name: String,
    pub collateral: bool,
    pub usd_fungible: bool,
    pub is_etf: bool,
    pub is_token: bool,
    pub hidden: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub can_convert: bool,
    pub has_tag: bool,
    pub collateral_weight: Decimal,
    pub fiat: bool,
    pub methods: Vec<String>,
    pub erc20_contract: Option<String>,
    pub bep2_asset: Option<String>,
    pub trc20_contract: Option<String>,
    pub spl_mint: Option<String>,
    pub credit_to: Option<String>,
    pub spot_margin: bool,
    pub nft_quote_currency_eligible: bool,
    pub index_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coins {
    pub success: bool,
    pub(crate) result: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceInfo {
    pub coin: String,
    pub total: Decimal,
    pub free: Decimal,
    pub available_without_borrow: Decimal,
    pub usd_value: Decimal,
    pub spot_borrow: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultData<T> {
    pub success: bool,
    pub(crate) result: T,
    pub has_more_data: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    address: String,
    tag: Option<String>,
    method: String,
    coin: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub id: i64,
    pub coin: String,
    pub txid: Option<String>,
    pub address: Option<Address>,
    pub size: Decimal,
    pub fee: Option<Decimal>,
    pub status: String,
    pub time: String,
    pub sent_time: Option<String>,
    pub confirmed_time: Option<String>,
    pub confirmations: Option<i64>,
    pub method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdrawal {
    pub id: i64,
    pub coin: String,
    pub address: Option<String>,
    pub tag: Option<String>,
    pub method: Option<String>,
    pub txid: Option<String>,
    pub size: Decimal,
    pub fee: Option<Decimal>,
    pub status: String,
    pub time: String,
    pub notes: Option<String>,
    pub destination_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Airdrops {
    pub coin: String,
    pub id: i64,
    pub size: Decimal,
    pub time: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    id: i64,
    client_id: Option<String>,
    market: String,
    #[serde(rename = "type")]
    option_type: String,
    side: String,
    price: Decimal,
    size: Decimal,
    status: String,
    filled_size: Decimal,
    remaining_size: Decimal,
    reduce_only: bool,
    liquidation: Option<bool>,
    avg_fill_price: Option<Decimal>,
    post_only: bool,
    ioc: bool,
    created_at: String,
    future: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrderInfo {
    id: i64,
    market: String,
    future: Option<String>,
    side: String,
    #[serde(rename = "type")]
    option_type: String,
    order_price: Option<Decimal>,
    trigger_price: Decimal,
    size: Decimal,
    status: String,
    created_at: String,
    triggered_at: Option<String>,
    order_id: Option<String>,
    error: Option<String>,
    reduce_only: bool,
    trail_value: Option<Decimal>,
    trail_start: Option<Decimal>,
    cancelled_at: Option<Decimal>,
    cancel_reason: Option<String>,
    retry_until_filled: bool,
    order_type: String,
    filled_size: Decimal,
    avg_fill_price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FillInfo {
    id: i64,
    market: String,
    future: Option<String>,
    base_currency: String,
    quote_currency: String,
    #[serde(rename = "type")]
    option_type: String,
    side: String,
    price: Decimal,
    size: Decimal,
    order_id: i64,
    time: String,
    trade_id: i64,
    fee_rate: Decimal,
    fee: Decimal,
    fee_currency: String,
    liquidity: String,
}
