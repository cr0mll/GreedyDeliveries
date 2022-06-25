use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpSocket, TcpStream},
};

use std::sync::Arc;
use std::sync::Mutex;

pub struct Server {
    listener: TcpListener,
    connections: Mutex<Vec<TcpStream>>,
    on_connection: fn(&mut TcpStream) -> ()
}

impl Server {
    pub const LISTEN_PORT: u16 = 1337;

    pub async fn new(on_connection: fn(&mut TcpStream) -> ()) -> Arc<Server> {
        Arc::new(Server {
            listener: TcpListener::bind(format!("0.0.0.0:{}", Server::LISTEN_PORT))
                .await
                .unwrap(),
            connections: Mutex::new(Vec::new()),
            on_connection
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
                        this.process_connection(conn);
                    }
                });
            } else {
                println!("Failed to establish connection!");
            }
        }
    }

    fn process_connection(self: Arc<Self>, mut connection: TcpStream) {
        (self.on_connection)(&mut connection);
        if let Ok(mut lock) = self.connections.lock() {
            lock.push(connection);
        } else {
            println!("Failed to obtain mutex lock.");
        }
    }
}
