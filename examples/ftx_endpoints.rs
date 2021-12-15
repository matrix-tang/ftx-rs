use chrono::Local;
use ftx_rs::api::*;

fn main() {
    println!("helo");

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

    let subaccounts = api.subaccounts.get_subaccounts();
    match subaccounts {
        Ok(s) => {
            println!("subaccounts: {:?}", s);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let subaccount_info = api.subaccounts.create_subaccount("nick1".into());
    match subaccount_info {
        Ok(info) => {
            println!("subaccount info: {:?}", info);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let change_result = api
        .subaccounts
        .change_subaccount_name("nick1".into(), "nick2".into());
    match change_result {
        Ok(r) => {
            println!("change name result: {:?}", r);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let del_result = api.subaccounts.delete_subaccount("nick1".into());
    match del_result {
        Ok(d) => {
            println!("del name result: {:?}", d);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let subaccount_balances = api.subaccounts.get_subaccount_balances("nick1".into());
    match subaccount_balances {
        Ok(b) => {
            println!("subaccount balances: {:?}", b);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let transfer_info = api.subaccounts.transfer_subaccounts(
        "USDT".to_string(),
        10 as f64,
        "main".to_string(),
        "flow".to_string(),
    );
    match transfer_info {
        Ok(ti) => {
            println!("transfer info: {:?}", ti);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
