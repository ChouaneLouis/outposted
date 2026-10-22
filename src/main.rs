use std::sync::Arc;

use tokio::sync::watch;

use outposted::game::GameState;
use outposted::net::run_tcp_listener;

#[tokio::main]
async fn main() {
    let game_state = GameState { tick: 0 };
    let (state_tx, state_rx) = watch::channel(Arc::new(game_state.clone()));
    tokio::spawn(async move { game_state.mainloop(state_tx).await });

    run_tcp_listener(state_rx).await.unwrap();
}
