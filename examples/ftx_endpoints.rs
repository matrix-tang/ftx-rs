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
}
