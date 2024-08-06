use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::SinkExt;
use tokio::sync::mpsc;
use crate::{Message, MessageCodec};
use std::time::Duration;
use std::io::ErrorKind;

pub struct Client {
    addr: String,
    receiver: mpsc::Receiver<Message>,
}

impl Client {
    pub fn new(addr: &str, receiver: mpsc::Receiver<Message>) -> Self {
        Client {
            addr: addr.to_string(),
            receiver,
        }
    }

    pub async fn start(mut self) {
        let addr = self.addr.clone();
        let (msg_tx, mut msg_rx) = mpsc::channel::<Message>(100);

        // Task to manage the socket connection
        tokio::spawn(async move {
            let mut pending_messages = vec![];

            loop {
                match TcpStream::connect(&addr).await {
                    Ok(socket) => {
                        let mut framed = Framed::new(socket, MessageCodec);

                        // Resend pending messages
                        for message in pending_messages.drain(..) {
                            if let Err(e) = framed.send(message).await {
                                if e.kind() == ErrorKind::BrokenPipe {
                                    eprintln!("Broken pipe while resending, reconnecting to {}: {:?}", addr, e);
                                    break;
                                } else {
                                    eprintln!("Failed to resend message to {}: {:?}", addr, e);
                                }
                            } else {
                                println!("Pending message sent to {}", addr);
                            }
                        }

                        while let Some(message) = msg_rx.recv().await {
                            if let Err(e) = framed.send(message.clone()).await {
                                if e.kind() == ErrorKind::BrokenPipe {
                                    eprintln!("Broken pipe, attempting to reconnect to {}: {:?}", addr, e);
                                    pending_messages.push(message);
                                    break; // Exit the loop to reconnect
                                } else {
                                    eprintln!("Failed to send message to {}: {:?}", addr, e);
                                }
                            } else {
                                println!("Message sent to {}", addr);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to connect to {}: {:?}", addr, e);
                    },
                }

                // Wait before attempting to reconnect
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        // Task to receive messages and forward them to the socket handling task
        tokio::spawn(async move {
            while let Some(message) = self.receiver.recv().await {
                let msg_tx_clone = msg_tx.clone();
                tokio::spawn(async move {
                    if let Err(e) = msg_tx_clone.send(message).await {
                        eprintln!("Failed to forward message: {:?}", e);
                    }
                });
            }
        });
    }
}
