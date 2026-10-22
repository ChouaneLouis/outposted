use std::io;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{WebSocketStream, accept_async};

use crate::game::GameState;

pub async fn run_tcp_listener(state_rx: watch::Receiver<Arc<GameState>>) -> io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listner.accept().await?;
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("websocket handshake failed: {e}");
                continue;
            },
        };

        let state_rx = state_rx.clone();
        tokio::spawn(async move { run_client_connection(ws_stream, state_rx).await });
    }
}

async fn run_client_connection(
    ws_stream: WebSocketStream<TcpStream>,
    mut state_rx: watch::Receiver<Arc<GameState>>,
) {
    println!("opening websocket.");
    let (mut ws_tx, mut ws_rx) = ws_stream.split();

    loop {
        tokio::select! {
            ws_next = ws_rx.next() => match ws_next {
                Some(Ok(Message::Close(_))) => break,
                Some(Ok(_)) => continue,
                Some(Err(e)) => {
                    eprintln!("websocket error: {e}");
                    break;
                },
                None => {
                    println!("websocket eof, closing gracefully.");
                    break;
                },
            },
            state_changed = state_rx.changed() => match state_changed {
                Ok(()) => {
                    let game_state = state_rx.borrow_and_update().clone();
                    println!("new game state arrived with tick {}", game_state.tick);
                },
                Err(e) => {
                    println!("game state communication has closed: {e}");
                    break;
                }
            },
        }
    }

    if let Err(e) = ws_tx.close().await {
        eprintln!("error closing the websocket: {e}");
    };
    println!("websocket closed.");
}
