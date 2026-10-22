use std::sync::Arc;

use serde::Serialize;
use tokio::sync::watch;
use tokio::time::{Duration, interval, MissedTickBehavior};

#[derive(Serialize, Clone, Debug)]
pub struct GameState {
    pub tick: i32,
}

impl GameState {
    pub async fn mainloop(mut self, state_tx: watch::Sender<Arc<GameState>>) {
        let mut intv = interval(Duration::from_millis(500));
        intv.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            self.tick += 1;

            intv.tick().await;
            state_tx.send(Arc::new(self.clone())).unwrap();
        }
    }
}
