mod module_controllers;
use std::{ thread};
use std::time::Duration;
use crate::module_controllers::{RelayHid, WinchState};
use axum::{routing::{get, post}, Router, response::Html, Extension};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let mut device = module_controllers::RelayHid::connect(0x16c0, 0x05df);
    let app = Router::new()
        .route("/", get(show_web_page))
        .route("/up", post(move_up)).layer(Extension(&mut device))
        .route("/down", post(move_down)).layer(Extension(&mut device))
        .route("/stop", post(stop_motor)).layer(Extension(& mut device));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();


    // device.set_state(WinchState::Winding);
    // thread::sleep(Duration::from_secs(1));
    // device.set_state(WinchState::Idle);
}
async fn show_web_page() {
    _
}
async fn move_up(Extension(device): &mut Extension<RelayHid>) -> String {
    device.set_state(WinchState::Idle);
    thread::sleep(Duration::from_millis(1000));
    device.set_state(WinchState::Winding);
    "moving up".to_string()
}
async fn move_down(Extension(device): &mut Extension<RelayHid> ) -> String {
    device.set_state(WinchState::Idle);
    thread::sleep(Duration::from_millis(1000));
    device.set_state(WinchState::Unwinding);
    "moving down".to_string()
}

async fn stop_motor(Extension(device): &mut Extension<RelayHid> ) -> String {
    device.set_state(WinchState::Idle);
    "stop moving motor".to_string()
}

