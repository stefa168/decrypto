use axum::response::IntoResponse;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;

use axum_typed_websockets::{Message, WebSocket, WebSocketUpgrade};

use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

use crate::messages::{ClientMessage, ServerMessage};

static COUNTER: AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(ws_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade<ServerMessage, ClientMessage>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket<ServerMessage, ClientMessage>, who: SocketAddr) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Err(e) => {
                println!("client {who} disconnected ({e})");
                return;
            }
            Ok(msg) => match msg {
                Message::Item(m) => match m {
                    ClientMessage::StatePls => todo!(),
                    ClientMessage::UpdateState(_) => todo!(),
                },
                Message::Ping(p) => todo!(),
                Message::Pong(p) => todo!(),
                Message::Close(c) => return,
            },
        }
    }

    // This second task will receive messages from client and print them on server console
    // let mut recv_task = tokio::spawn(async move {
    //     let mut cnt = 0;
    //     while let Some(Ok(msg)) = receiver.next().await {
    //         cnt += 1;
    //         // print message and break if instructed to do so
    //         if process_message(msg, who).is_break() {
    //             break;
    //         }
    //     }
    //     cnt
    // });

    println!("Websocket context {who} destroyed");
}
