use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}
};

use std::sync::Arc;
use tokio::sync::broadcast;

pub use super::{Message, MessageHeader, MessageType};

pub struct Server {
    listener: TcpListener,
    sender: broadcast::Sender<Message>
}

impl Server {
    pub const LISTEN_PORT: u16 = 1337;

    pub async fn new() -> Arc<Server> {
        // There shouldn't be many messages broadcast by the server
        let (sender, _) = broadcast::channel(16);
        let instance = Arc::new(Server {
            listener: TcpListener::bind(format!("0.0.0.0:{}", Server::LISTEN_PORT))
                .await
                .unwrap(),
                sender
        });

        instance
    }

    pub async fn run(self: Arc<Self>) {  
        self.listen().await;
    }

    async fn listen(self: Arc<Self>) {
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
        let mut broadcast_receiver = self.sender.subscribe();
        let mut msg_body = vec![1; 12];
        msg_body.shrink_to_fit();
        let test_message = Message {
            header: MessageHeader { message_type: MessageType::PostBlockchain, message_size: 12 },
            body: msg_body
        };
        
        self.broadcast(test_message).await;

        loop {
            // First send out any broadcasts
            if let Ok(message) = broadcast_receiver.recv().await {
                self.send_message(message, &mut connection).await;
            } else {
                println!("Failed to broadcast message!");
            }

            let msg = self.read_message(&mut connection).await;

            match msg.header.message_type {
                _ => ()
            }
        }
    }

    async fn broadcast(&self, message: Message) {
        match self.sender.send(message) {
            Ok(_) => (),
            Err(_) => println!("Failed to send message through channel!")
        }
    }

    async fn send_message(&self, message: Message, client: &mut TcpStream) {

        let message_bytes = message.serialise();
        client.write_all(&message_bytes[..]).await.unwrap();
    }

    async fn read_message(&self, client: &mut TcpStream) -> Message {
        let mut msg_header: [u8; 40] = [0; 40];

        if let Err(err) = client.read_exact(&mut msg_header).await {
            println!("An error occurred: {}", err.to_string());
        }
        let msg_header: MessageHeader = bincode::deserialize(&msg_header).unwrap();

        let mut msg_body: Vec<u8> = Vec::with_capacity(msg_header.message_size as usize);
        if let Err(err) = client.read_exact(&mut msg_body[..]).await {
            println!("An error occurred: {}", err.to_string());
        }

        Message::deserialise_with_header(&msg_body[..], msg_header)
    }
}
