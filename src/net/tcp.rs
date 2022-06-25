use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpSocket, TcpStream},
};

use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};

pub use super::{Message, MessageHeader, MessageType};

pub struct Server {
    listener: TcpListener,
    connections: Mutex<Vec<TcpStream>>,
}

impl Server {
    pub const LISTEN_PORT: u16 = 1337;

    pub async fn new() -> Arc<Server> {
        Arc::new(Server {
            listener: TcpListener::bind(format!("0.0.0.0:{}", Server::LISTEN_PORT))
                .await
                .unwrap(),
            connections: Mutex::new(Vec::new())
        })
    }

    pub async fn listen(self: Arc<Self>) {
        println!("Listening for connections...");
        loop {
            if let Ok((conn, _)) = self.listener.accept().await {
                
                println!("Processing connection!");
                tokio::spawn({
                    let this = Arc::clone(&self);
                    async move {
                        this.process_connection(conn).await;
                    }
                });
            } else {
                println!("Failed to establish connection!");
            }
        }
    }

    async fn process_connection(self: Arc<Self>, mut connection: TcpStream) {
        let mut lock = self.connections.lock().await;

            let msg = Message {
                header: MessageHeader {
                    message_type: MessageType::ReturnActiveSubscriptions,
                    message_size: 12
                },
                body: Vec::new()
            };

            self.send_message(msg, &mut connection).await;
            
        lock.push(connection);
    }

    pub async fn send_message(&self, message: Message, client: &mut TcpStream) {
        let message_bytes = bincode::serialize(&message).unwrap();

        client.write_all(&message_bytes[..]).await.unwrap();
    }
}
