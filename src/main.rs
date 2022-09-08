
use std::fs;

use futures_util::StreamExt;
use warp::Filter;
use warp::ws::{Message, WebSocket};

mod poly;

fn get_data() -> String {
    fs::read_to_string("data/index.html").unwrap()
}
fn message_data(msg: &Message) -> Option<&[u8]>{
    if msg.is_text() {
        println!("message: {}", msg.to_str().unwrap());
    }
    else if msg.is_binary() {
        return Some(msg.as_bytes());
    }
    return None;
}

async fn comm(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    let thread = tokio::spawn(async move {poly::send_test_poly(&mut tx).await});
    while let Some(msg) = rx.next().await {
        match msg {
            Ok(msg) => {
                let data = message_data(&msg);
                if let Some(_) = data {
                    println!("Recieved binary data");
                }
            },
            Err(e) => {
                eprintln!("Websocket error: {}", e);
            }
        };
    }
    thread.await.unwrap();
}

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(get_data).map(|data| warp::reply::html(data));

    let sock = warp::path("sock")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| 
            ws.on_upgrade(|socket| comm(socket))
        );
    
    // macro for multiple or's?
    let routes = index.or(sock);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
