use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(tag="type")]
struct GameObject {
    x_pos: f32,
    y_pos: f32,
    radius: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let object_a = GameObject {
        x_pos: 95.0,
        y_pos: 50.0,
        radius: 40.0,
    };
    let object_b = GameObject {
        x_pos: 0.0,
        y_pos: 0.0,
        radius: 7.0,
    };
    
    let map: Arc<[GameObject]> = [object_a, object_b].into();


    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Tcp listener running.");

    while let Ok((stream, _)) = listener.accept().await {
        let map = map.clone();

        tokio::spawn(async move {
            let ws = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(error) => {
                    eprintln!("Websocket handshake failed: {error}");
                    return;
                },
            };
            println!("Websocket connection established.");

            let (mut writer, mut reader) = ws.split();

            match serde_json::to_string(&*map) {
                Ok(json_data) => {
                    if let Err(error) = writer.send(Message::Text(json_data.into())).await {
                        eprintln!("Websocket send error: {error}");
                        return;
                    }
                    println!("Map sent to the client.");
                },
                Err(error) => { eprintln!("Failed to serialize map: {error}")},
            }
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
