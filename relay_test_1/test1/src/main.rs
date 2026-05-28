use axum::routing::get;

use socketioxide::{
    extract::{SocketRef, Data},
    SocketIo,
};

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |socket: SocketRef| {
        println!("Client connected: {:?}", socket.id);

        socket.on(
            "message",
            |socket: SocketRef, Data::<String>(data)| async move {
                println!("Received message: {}", data);

                let _ = socket.emit("received", "hello");
            },
        );
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "socket io server" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}