
use tokio::net::{TcpListener, TcpSocket, TcpStream};
pub struct Blockchain {
    listener: TcpListener,
    connections: Vec<TcpStream>
}

impl Blockchain {
    pub const LISTEN_PORT: u16 = 1337;

    pub async fn new() -> Blockchain {
        Blockchain {
            listener: TcpListener::bind(format!("0.0.0.0:{}", Blockchain::LISTEN_PORT))
                .await
                .unwrap(),
            connections: Vec::new()
                
        }
    }

    pub async fn listen(&mut self) {
        println!("Listening for connections...");
        loop {
            if let Ok((conn, _)) = self.listener.accept().await {
                println!("Connection established!");
                self.connections.push(conn);
            } else {
                println!("Failed to establish connection!");
            }
        }
    }
}
