use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use tokio_util::codec::Framed;
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use crate::{Message, Protocol, MessageCodec};

pub struct Server {
    addr: String,
    sender: mpsc::Sender<Message>,
}

impl Server {
    pub fn new(addr: &str, sender: mpsc::Sender<Message>) -> Self {
        Server {
            addr: addr.to_string(),
            sender,
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", &self.addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let sender = self.sender.clone();
            tokio::spawn(Self::handle_client(socket, sender));
        }
    }

    async fn handle_client(socket: TcpStream, sender: mpsc::Sender<Message>) -> Result<(), io::Error> {
        let mut framed = Framed::new(socket, MessageCodec);

        while let Some(result) = framed.next().await {
            match result {
                Ok(message) => {
                    match message.payload {
                        Protocol::Json(_) => {
                            println!("Received JSON");
                            if let Err(e) = sender.send(message).await {
                                println!("Failed to forward message to DataSink: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to decode message: {:?}", e);
                }
            }
        }

        Ok(())
    }
}
