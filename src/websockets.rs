use ring::hmac;
use std::net::TcpStream;
use tungstenite::handshake::client::Response;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

use std::sync::mpsc::{self, channel};
use url::Url;

use crate::errors::*;
use crate::events::*;
use chrono::Local;

static SUBSCRIBED: &'static str = "subscribed";
static INFO: &'static str = "info";
static ERROR: &'static str = "error";
static PONG: &'static str = "pong";
static WEBSOCKET_URL: &'static str = "wss://ftx.com/ws/";

pub trait EventHandler {
    fn on_connect(&mut self, event: NotificationEvent);
    fn on_auth(&mut self, event: NotificationEvent);
    fn on_subscribed(&mut self, event: NotificationEvent);
    fn on_data_event(&mut self, event: DataEvent);
    fn on_error(&mut self, message: Error);
}

#[derive(Debug)]
enum WsMessage {
    Close,
    Text(String),
}

pub struct WebSockets {
    api_key: String,
    secret_key: String,
    socket: Option<(WebSocket<MaybeTlsStream<TcpStream>>, Response)>,
    sender: Sender,
    rx: mpsc::Receiver<WsMessage>,
    event_handler: Option<Box<dyn EventHandler>>,
    login_status: bool,
}

impl WebSockets {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> WebSockets {
        let (tx, rx) = channel::<WsMessage>();
        let sender = Sender { tx: tx };

        WebSockets {
            api_key: api_key.unwrap_or("".into()),
            secret_key: secret_key.unwrap_or("".into()),
            socket: None,
            sender: sender,
            rx: rx,
            event_handler: None,
            login_status: false,
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        let wss: String = format!("{}", WEBSOCKET_URL);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                bail!(format!("Error during handshake {}", e))
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            return Ok(());
        }
        bail!("Not able to close the connection");
    }

    pub fn add_event_handler<H>(&mut self, handler: H)
    where
        H: EventHandler + 'static,
    {
        self.event_handler = Some(Box::new(handler));
    }

    pub fn ping(&mut self) {
        let msg = json!({
            "op": "ping",
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_ticker<S>(&mut self, symbol: S)
    where
        S: Into<String>,
    {
        let msg = json!({
            "op": "subscribe",
            "channel": "ticker",
            "market": symbol.into(),
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_trades<S>(&mut self, symbol: S)
    where
        S: Into<String>,
    {
        let msg = json!({
            "op": "subscribe",
            "channel": "trades",
            "market": symbol.into(),
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_orderbook_grouped<S>(&mut self, symbol: S, group: i64)
    where
        S: Into<String>,
    {
        let msg = json!({
            "op": "subscribe",
            "channel": "orderbookGrouped",
            "market": symbol.into(),
            "grouping": group,
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_orderbook<S>(&mut self, symbol: S)
    where
        S: Into<String>,
    {
        let msg = json!({
            "op": "subscribe",
            "channel": "orderbook",
            "market": symbol.into(),
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn unsubscribe(&mut self, channel: String, symbol: Option<String>, grouping: Option<i64>) {
        let mut msg = json!({
            "op": "unsubscribe",
            "channel": channel,
        });
        if let Some(ref market) = symbol {
            msg = json!({
                "op": "unsubscribe",
                "channel": channel,
                "market": market,
            });
        }
        if let Some(g) = grouping {
            msg = json!({
                "op": "unsubscribe",
                "channel": channel,
                "market": &symbol.unwrap(),
                "grouping": g,
            });
        }
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn login(&mut self) {
        let ts = Local::now().timestamp() * 1000;
        let signature_payload = format!("{}websocket_login", ts);
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hex::encode(hmac::sign(&signed_key, signature_payload.as_bytes()).as_ref());

        let msg = json!({
            "op": "login",
            "args": {
                "key": self.api_key,
                "sign": signature,
                "time": ts,
            },
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
        self.login_status = true;
    }

    pub fn subscribe_fills(&mut self) {
        if !self.login_status {
            self.login();
        }

        let msg = json!({
            "op": "subscribe",
            "channel": "fills",
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_orders(&mut self) {
        if !self.login_status {
            self.login();
        }

        let msg = json!({
            "op": "subscribe",
            "channel": "orders",
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn subscribe_ftxpay(&mut self) {
        if !self.login_status {
            self.login();
        }

        let msg = json!({
            "op": "subscribe",
            "channel": "ftxpay",
        });
        if let Err(e) = self.sender.send(&msg.to_string()) {
            println!("{:?}", e);
        }
    }

    pub fn event_loop(&mut self) -> Result<()> {
        let mut ping_flag = 0;
        loop {
            if let Some(ref mut socket) = self.socket {
                loop {
                    match self.rx.try_recv() {
                        Ok(msg) => match msg {
                            WsMessage::Text(text) => {
                                socket.0.write_message(Message::Text(text))?;
                            }
                            WsMessage::Close => {
                                return socket.0.close(None).map_err(|e| e.into());
                            }
                        },
                        Err(mpsc::TryRecvError::Disconnected) => {
                            bail!("Disconnected")
                        }
                        Err(mpsc::TryRecvError::Empty) => break,
                    }
                }

                let message = socket.0.read_message()?;

                match message {
                    Message::Text(text) => {
                        if let Some(ref mut h) = self.event_handler {
                            if text.find(INFO) != None {
                                println!("INFO: {:?}", text);
                            } else if text.find(SUBSCRIBED) != None {
                                let event: NotificationEvent = serde_json::from_str(&text)?;
                                h.on_subscribed(event);
                            } else if text.find(ERROR) != None {
                                println!("ERROR: {:?}", text);
                            } else if text.find(PONG) != None {
                                let event: NotificationEvent = serde_json::from_str(&text)?;
                                h.on_subscribed(event);
                            } else {
                                // println!("text: {:?}", &text);
                                let event: DataEvent = serde_json::from_str(&text)?;
                                h.on_data_event(event);
                            }
                        }
                    }
                    Message::Binary(_) => {}
                    Message::Ping(_) | Message::Pong(_) => {}
                    Message::Close(e) => {
                        bail!(format!("Disconnected {:?}", e));
                    }
                }

                ping_flag = ping_flag + 1;
                if ping_flag >= 20 {
                    ping_flag = 0;
                    self.ping();
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<WsMessage>,
}

impl Sender {
    pub fn send(&self, raw: &str) -> Result<()> {
        self.tx
            .send(WsMessage::Text(raw.to_string()))
            .map_err(|e| Error::with_chain(e, "not able to send a message"))?;
        Ok(())
    }

    pub fn shutdown(&self) -> Result<()> {
        self.tx
            .send(WsMessage::Close)
            .map_err(|e| Error::with_chain(e, "Error during shutdown"))?;
        Ok(())
    }
}
