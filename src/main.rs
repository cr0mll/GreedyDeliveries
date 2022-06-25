mod net;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut blockchain = net::tcp::Server::new().await;
    
    blockchain.listen().await;
    
    println!("Hello, world!");
}
