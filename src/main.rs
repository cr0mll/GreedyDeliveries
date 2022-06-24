mod net;

#[tokio::main]
async fn main() {
    let mut blockchain = net::Blockchain::new().await;
    blockchain.listen().await;
    println!("Hello, world!");
}
