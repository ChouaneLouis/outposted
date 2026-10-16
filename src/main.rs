use futures_util::StreamExt;
use std::error::Error;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Tcp listener running.");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(error) => {
                    eprintln!("Websocket handshake failed: {error}");
                    return;
                },
            };
            println!("Websocket connection established.");

            let (_, mut reader) = ws.split();
            while let Some(message) = reader.next().await {
                match message {
                    Ok(content) => {
                        println!("Message recieved: {content:?}");

                        if content.is_close() {
                            println!("Client requested connection closure.");
                            break;
                        }
                    },
                    Err(error) => {
                        eprintln!("Websocket error: {error}");
                        break;
                    },
                }
            }
        });
    }

    Ok(())
}
