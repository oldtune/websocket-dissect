use std::net::SocketAddr;

use futures::SinkExt;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:5001").await.unwrap();
    while let Ok((stream, peer_address)) = tcp_listener.accept().await {
        tokio::spawn(async move {
            spawn_websocket(stream, peer_address).await;
        });
    }
}

async fn spawn_websocket(tcp_stream: TcpStream, peer_addr: SocketAddr) {
    println!("New connection request from {}", peer_addr.ip().to_string());
    let websocket = accept_async(tcp_stream).await.unwrap();
    let (mut sending, mut incoming) = websocket.split();
    while let Some(msg) = incoming.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let text = msg.into_text().unwrap();
                println!("Received {text}");
                let message = Message::text(&text);
                sending.send(message).await.unwrap();
                println!("echo back {text}");
            } else if msg.is_close() {
                println!("Received close message!");
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }
}
