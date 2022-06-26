mod net;

#[tokio::main]
async fn main() {
    let blockchain = net::tcp::Server::new().await;
    blockchain.run().await;
    
    println!("Hello, world!");
}
