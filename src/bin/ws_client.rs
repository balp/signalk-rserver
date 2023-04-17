#![allow(clippy::uninlined_format_args)]

use std::str;

use awc::Client;
use awc::error::WsProtocolError;
use awc::ws::Frame;
use bytes::Bytes;
use futures_util::stream::StreamExt;
use serde::Deserialize;
use serde_json::Value;

use signalk_rserver::signalk::{
    SignalKStreamMessage, Storage, V1DeltaFormat, V1FullFormat, V1Hello,
};

#[derive(Debug)]
enum SignalKWSState {
    Disconnected,
    Connected,
}

struct SignalKUpdater {
    state: SignalKWSState,
    storage: Storage,
}
impl Default for SignalKUpdater {
    fn default() -> Self {
        SignalKUpdater {
            state: SignalKWSState::Disconnected,
            storage: Storage::default(),
        }
    }
}
impl SignalKUpdater {
    pub fn handle_ws_frame(&mut self, result: Result<Frame, WsProtocolError>) {
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
        match serde_json::from_str(str_message) {
            Ok(message) => {
                match message {
                    SignalKStreamMessage::Hello(msg) => {
                        self.state = SignalKWSState::Connected;
                        if let Some(ref value) = msg.self_ {
                            self.storage.set_self(&value);
                        }
                    }
                    SignalKStreamMessage::Delta(msg) => self.storage.update(&msg),
                    _ => (),
                };
                ()
            }
            Err(_) => {
                let v: Value = serde_json::from_str(str_message).unwrap();
                println!("    value: {:?}", v);
                ()
            }
        };
        println!("    to: {:?}", self.storage.get());
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

    for _ in 0..50 {
        let option = connection.next().await;
        let res_1 = option.unwrap();
        sk_handler.handle_ws_frame(res_1);
    }
}
