use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use url::form_urlencoded;

#[tokio::main]
async fn main() {

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |socket: SocketRef| {
        if let Some(query_str) = socket.req_parts().uri.query() {

            let mut device_id = None;
            for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {
                if key == "deviceId" {
                    device_id = Some(value.into_owned());
                    break;
                }
            }

            if let Some(id) = device_id {
                println!("connected device: {}", id);

                let _ = socket.join(id);

            } else {
                println!("connection closed, does not exit parametr deviceId");
                let _ = socket.disconnect();
            }
        } else {
            println!("connection closed no querry parametrs");
            let _ = socket.disconnect();
        }
    });


    let app = Router::new()
        .route("/", get(show_index))
        .route("/api/control/:device_id/:command", get(http_control_handler))
        .layer(layer)
        .with_state(io);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("connected");

    axum::serve(listener, app).await.unwrap();
}

async fn http_control_handler(
    Path((device_id, command)): Path<(String, String)>,
    State(io): State<SocketIo>,
) -> &'static str {
    println!("device [{}] -> command: [{}]", device_id, command);

    let _ = io.to(device_id).emit("command", command);

    "sended"
}

async fn show_index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}