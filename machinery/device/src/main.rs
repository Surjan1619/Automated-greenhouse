use rust_socketio::{
    asynchronous::{Client, ClientBuilder},
    Payload,
};
use std::sync::Arc;            // Добавили для многопоточного владения
use tokio::sync::Mutex;
use std::time::Duration;
use crate::module_controller::{RelayHid, WinchState};

mod module_controller;
#[tokio::main]
async fn main() {
    let mut device = Arc::new(Mutex::new(module_controller::RelayHid::connect(0x16c0, 0x05df)));
    let mut device_for_handler = Arc::clone(&device);
    let url = "http://16.16.24.244/";
    let device_id = "winch123";
    let full_url = format!("{}?deviceId={}", url, device_id);
    let socket = ClientBuilder::new(full_url)
        .namespace("/")
        .on("connect", |_, _| Box::pin(async move {
            println!("Connected to server");
        }))
        .on("disconnect", |_, _| Box::pin(async move {
            println!("Disconnected from server");
        }))
        .on("command", move |payload: Payload, _| {
            let device = Arc::clone(&device_for_handler);
            Box::pin(async move {
                handle_incoming_command(payload, device).await;
            })
        })
        .connect()
        .await;
    match socket {
        Ok(socket) => {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

async fn handle_incoming_command(payload: Payload, device: Arc<Mutex<module_controller::RelayHid>>) {
    let command_txt = match payload {
        // Новые версии rust-socketio используют Payload::Text
        Payload::Text(values) => {
            values.get(0)
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        }
        // Старые версии
        Payload::String(s) => Some(s),
        _ => None,
    };

    if let Some(command_txt) = command_txt {
        println!("Command: {}", command_txt);
        let mut device = device.lock().await;
        match command_txt.as_str() {
            "UP"   => device.set_state(WinchState::Winding),
            "DOWN" => device.set_state(WinchState::Unwinding),
            "STOP" => device.set_state(WinchState::Idle),
            _      => println!("Unknown command: {}", command_txt),
        }
    } else {
        println!("Could not parse payload");
    }
}



