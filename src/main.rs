use outposted::net::run_tcp_listener;

#[tokio::main]
async fn main() {
    run_tcp_listener().await.unwrap();
}
