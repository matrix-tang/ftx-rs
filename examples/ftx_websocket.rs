use ftx_rs::errors::*;
use ftx_rs::events::*;
use ftx_rs::websockets::*;

struct WebSocketHandler;

impl EventHandler for WebSocketHandler {
    fn on_connect(&mut self, _event: NotificationEvent) {}

    fn on_auth(&mut self, _event: NotificationEvent) {}

    fn on_subscribed(&mut self, event: NotificationEvent) {
        match event {
            NotificationEvent::Subscribed(msg) => {
                println!("Subscribed: {:?}", msg);
            }
            NotificationEvent::Pong(msg) => {
                println!("Pong: {:?}", msg);
            }
        }
    }

    fn on_data_event(&mut self, event: DataEvent) {
        match event {
            DataEvent::TickerEvent(ticker_data) => {
                println!("ticker: {:?}", ticker_data);
            }
            DataEvent::TradesEvent(trades_data) => {
                println!("trades: {:?}", trades_data);
            }
            DataEvent::OrderBookEvent(orderbook_data) => {
                println!("orderbook: {:?}", orderbook_data);
            }
            _ => {}
        }
    }

    fn on_error(&mut self, message: Error) {
        println!("{:?}", message);
    }
}

fn main() {
    let mut w = WebSockets::new(Some("api_key".into()), Some("secret_key".into()));
    w.add_event_handler(WebSocketHandler);
    w.connect().unwrap();
    w.ping();
    w.subscribe_trades("BTC-PERP");
    w.subscribe_orderbook("BTC-PERP");
    w.subscribe_ticker("BTC-PERP");
    w.subscribe_orderbook_grouped("BTC-PERP", 10);
    // w.subscribe_fills();
    // w.subscribe_orders();
    // w.subscribe_ftxpay();
    // w.unsubscribe("ticker".into(), Some("BTC-PERP".into()), None);
    // w.unsubscribe("fills".into(), None, None);
    // w.unsubscribe("orderbookGrouped".into(), Some("BTC-PERP".into()), Some(10));
    w.event_loop().unwrap();
}
