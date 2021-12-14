use crate::errors::*;
use chrono::Local;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::{self, Response, StatusCode};
use ring::hmac;
use std::io::Read;

static API_HOST: &'static str = "https://ftx.com/api";

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key: api_key.unwrap_or("".into()),
            secret_key: secret_key.unwrap_or("".into()),
            client: reqwest::Client::new(),
        }
    }

    pub fn get(&self, endpoint: String, request: String) -> Result<String> {
        let mut url: String = format!("{}{}", API_HOST, String::from(endpoint));
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }
        // let response = reqwest::get(url.as_str())?;
        let response = self.client.get(url.as_str()).send()?;

        self.handler(response)
    }

    pub fn get_signed(
        &self,
        endpoint: String,
        payload: String,
        subaccount: Option<String>,
    ) -> Result<String> {
        let url: String = format!("{}{}", API_HOST, String::from(&endpoint));
        let response = self
            .client
            .get(url.as_str())
            .headers(self.build_headers(
                String::from("GET"),
                endpoint,
                payload.clone(),
                subaccount,
            )?)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed(
        &self,
        endpoint: String,
        payload: String,
        subaccount: Option<String>,
    ) -> Result<String> {
        let url: String = format!("{}{}", API_HOST, String::from(&endpoint));
        // println!("{:?}", url);
        let response = self
            .client
            .post(url.as_str())
            .headers(self.build_headers(
                String::from("POST"),
                endpoint,
                payload.clone(),
                subaccount.clone(),
            )?)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    pub fn delete_signed(
        &self,
        endpoint: String,
        payload: String,
        subaccount: Option<String>,
    ) -> Result<String> {
        let url: String = format!("{}{}", API_HOST, String::from(&endpoint));
        let response = self
            .client
            .delete(url.as_str())
            .headers(self.build_headers(
                String::from("DELETE"),
                endpoint,
                payload.clone(),
                subaccount.clone(),
            )?)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    fn build_headers(
        &self,
        method: String,
        endpoint: String,
        payload: String,
        subaccount: Option<String>,
    ) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("ftx-rs"));
        headers.insert(
            HeaderName::from_static("ftx-key"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        // signature
        let ts = Local::now().timestamp() * 1000;
        let signature_payload = format!("{}{}/api{}{}", ts, method, endpoint, payload);
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hex::encode(hmac::sign(&signed_key, signature_payload.as_bytes()).as_ref());
        headers.insert(
            HeaderName::from_static("ftx-sign"),
            HeaderValue::from_str(signature.as_str())?,
        );
        headers.insert(HeaderName::from_static("ftx-ts"), HeaderValue::from(ts));

        // set content type
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // set sub account
        if let Some(s) = subaccount {
            headers.insert(
                HeaderName::from_static("ftx-subaccount"),
                HeaderValue::from_str(s.as_str())?,
            );
        }

        // println!("{}, {:?}", signature_payload, headers);

        Ok(headers)
    }

    fn handler(&self, mut response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                let mut body = String::new();
                response.read_to_string(&mut body)?;
                return Ok(body);
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                bail!(format!("Bad Request: {:?}", response));
            }
            s => {
                bail!(format!("Received response: {:?}", s));
            }
        };
    }
}
