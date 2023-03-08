#![allow(clippy::uninlined_format_args)]

// use std::io::Bytes;
// use actix_rt::System;
use awc::error::WsProtocolError;
use awc::ws::Frame;
use awc::Client;
use bytes::Bytes;
use futures_util::stream::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use signalk_rserver::signalk::delta::V1DeltaFormat;
use signalk_rserver::signalk::hello::V1Hello;
use std::str;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SignalKWSMessage {
    Hello(V1Hello),
    Delta(V1DeltaFormat),
    Dummy,
}

#[derive(Debug)]
enum SignalKWSState {
    Disconnected,
    Connected,
}

struct SignalKUpdater {
    state: SignalKWSState,
}
impl Default for SignalKUpdater {
    fn default() -> Self {
        SignalKUpdater {
            state: SignalKWSState::Disconnected,
        }
    }
}
impl SignalKUpdater {
    pub fn handle_ws_frame(&mut self, result: Result<Frame, WsProtocolError>) {
        println!("handle_ws_frame {:?}", result);
        let response = result.unwrap();
        match response {
            Frame::Text(text) => self.handle_text_message(&text),
            // Frame::Binary(text) => println!("binary: {}", str::from_utf8(&text).unwrap()),
            // Frame::Continuation(_item) => (),
            // Frame::Ping(text) => println!("ping: {}", str::from_utf8(&text).unwrap()),
            // Frame::Pong(text) => println!("pong: {}", str::from_utf8(&text).unwrap()),
            // Frame::Close(_reason) => println!("Close"),
            _ => {}
        }
    }

    fn handle_text_message(&mut self, text: &Bytes) {
        let str_message = str::from_utf8(&text).unwrap();
        println!("    state: {:?} text: {:?}", self.state, str_message);
        let message: SignalKWSMessage = match serde_json::from_str(str_message) {
            Ok(x) => x,
            Err(_) => {
                let v: Value = serde_json::from_str(str_message).unwrap();
                println!("    value: {:?}", v);
                SignalKWSMessage::Dummy
            }
        };
        println!("    to: {:?}", message);
        match message {
            SignalKWSMessage::Hello(a) => (self.state = SignalKWSState::Connected),
            _ => (),
        }
    }
}

#[actix_rt::main]
async fn main() {
    let mut sk_handler = SignalKUpdater::default();
    let (_resp, mut connection) = Client::new()
        .ws("ws://homenuc:3003/signalk/v1/stream?subscribe=self")
        .connect()
        .await
        .unwrap();

    for _ in 0..10 {
        let option = connection.next().await;
        let res_1 = option.unwrap();
        sk_handler.handle_ws_frame(res_1);
    }
}
