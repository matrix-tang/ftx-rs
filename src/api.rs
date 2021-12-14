use crate::account::*;
use crate::futures::*;
use crate::markets::*;
use crate::orders::*;
use crate::subaccounts::*;
use crate::wallet::*;

#[derive(Clone)]
pub struct Ftx {
    pub markets: MarketsClient,
    pub subaccounts: SubAccountsClient,
    pub futures: FuturesClient,
    pub account: AccountClient,
    pub wallet: WalletClient,
    pub orders: OrdersClient,
}

impl Ftx {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Ftx {
            markets: MarketsClient::new(),
            subaccounts: SubAccountsClient::new(api_key.clone(), secret_key.clone()),
            futures: FuturesClient::new(),
            account: AccountClient::new(api_key.clone(), secret_key.clone()),
            wallet: WalletClient::new(api_key.clone(), secret_key.clone()),
            orders: OrdersClient::new(api_key.clone(), secret_key.clone()),
        }
    }
}
