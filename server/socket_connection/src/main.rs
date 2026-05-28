mod Hid_Control;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Html,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::{Arc, Mutex};
use hidapi::HidDevice;
use tokio::sync::broadcast;
use crate::Hid_Control::{RelayHid, WinchState};

struct AppState {
    tx: broadcast::Sender<String>,
    device: Arc<Mutex<Hid_Control::RelayHid>>,
}

#[tokio::main]
async fn main() {
    
}