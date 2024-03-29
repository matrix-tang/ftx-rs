# ftx-rs

Unofficial Rust Library for the [Ftx API](https://docs.ftx.com/#overview)

<a href="https://crates.io/crates/ftx-rs" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/crates/v/ftx-rs" alt="Crates.io"></a>

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
ftx-rs = "0.1.6"
```

### REST
```rust
    let api = Ftx::new(Some("api_key".into()), Some("secret_key".into());

    // subaccounts
    // api.subaccounts.get_subaccounts
    // api.subaccounts.create_subaccount
    // api.subaccounts.change_subaccount_name
    // api.subaccounts.delete_subaccount
    // api.subaccounts.get_subaccount_balances
    // api.subaccounts.transfer_subaccounts

    // markets
    // api.markets.get_markets
    // api.markets.get_market
    // api.markets.get_orderbook
    // api.markets.get_trades
    // api.markets.get_historical_prices

    // futures
    // api.futures.get_futures
    // api.futures.get_future
    // api.futures.get_stats
    // api.futures.get_funding_rates
    // api.futures.get_indexes_weights
    // api.futures.get_expired_futures

    // account
    // api.account.get_account
    // api.account.get_positions
    // api.account.change_account_leverage

    // wallet
    // api.wallet.get_coins
    // api.wallet.get_balances
    // api.wallet.get_all_balances
    // api.wallet.get_deposit_address
    // api.wallet.get_deposits
    // api.wallet.get_withdrawals
    // api.wallet.get_airdrops

    // orders
    // api.orders.get_open_orders
    // api.orders.get_order_history
    // api.orders.get_open_trigger_orders
    // api.orders.place_order
    // api.orders.modify_order
    // api.orders.modify_order_by_client_id
    // api.orders.get_order_status
    // api.orders.get_order_status_by_client_id
    // api.orders.cancel_order
    // api.orders.cancel_order_by_client_id
    // api.orders.cancel_all_orders
    // ...

    // fills
    // api.fills.get_fills
        
        
```

### MARKET DATA

```rust
use chrono::Local;
use ftx_rs::api::*;

fn main() {
    let api = Ftx::new(None, None);

    let markets = api.markets.get_markets();
    match markets {
        Ok(m) => {
            println!("markets: {:?}", m);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let market = api.markets.get_market("BTC-PERP");
    match market {
        Ok(m) => {
            println!("market: {:?}", m);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let orderbook = api.markets.get_orderbook("BTC-PERP", 20);
    match orderbook {
        Ok(ob) => {
            println!("orderbook: {:?}", ob);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let trades = api.markets.get_trades("BTC-PERP");
    match trades {
        Ok(t) => {
            println!("trades: {:?}", t);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let candles = api
        .markets
        .get_historical_prices("BTC-PERP", 300, 0, Local::now().timestamp());
    match candles {
        Ok(c) => {
            println!("candles: {:?}", c);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // ...
}
```

### FTX_ENDPOINTS
API, see [example](https://github.com/matrix-tang/ftx-rs/blob/main/examples/ftx_endpoints.rs)

### FTX_WEBSOCKETS
WEBSOCKET, see [example](https://github.com/matrix-tang/ftx-rs/blob/main/examples/ftx_websockets.rs)