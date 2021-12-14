use ftx_rs::api::*;

fn main() {
    println!("helo");

    let api = Ftx::new(None, None);

    let markets = api.markets.ticker_all();
    match markets {
        Ok(m) => {
            println!("markets: {:?}", m);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let market = api.markets.ticker("BTC-PERP");
    match market {
        Ok(m) => {
            println!("market: {:?}", m);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
