#![allow(dead_code)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_json;
extern crate serde;

pub mod account;
pub mod api;
pub mod client;
pub mod errors;
pub mod events;
pub mod fills;
pub mod futures;
pub mod markets;
pub mod model;
pub mod orders;
pub mod subaccounts;
pub mod wallet;
pub mod websockets;
