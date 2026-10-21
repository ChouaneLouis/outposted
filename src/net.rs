use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{WebSocketStream, accept_async};
use std::io;

pub async fn run_tcp_listener() -> io::Result<()> {
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

        tokio::spawn(async move { run_client_connection(ws_stream).await });
    }
}

async fn run_client_connection(ws_stream: WebSocketStream<TcpStream>) {
    println!("opening websocket.");
    let (_, mut read) = ws_stream.split();

    loop {
        match read.next().await {
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
        }
    }

    println!("closing websocket.");
}
