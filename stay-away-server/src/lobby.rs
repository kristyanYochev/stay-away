use serde::Deserialize;
use tokio::sync::mpsc;

pub type LobbyHandle = mpsc::Sender<ClientMessage>;

#[derive(Debug, Deserialize)]
pub enum ClientMessage {}

pub struct Lobby;

impl Lobby {
    const BUFFER_SIZE: usize = 64;

    pub fn start() -> LobbyHandle {
        let (tx, mut rx) = mpsc::channel(Self::BUFFER_SIZE);

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                println!("Got Message: {:?}", msg);
            }
        });

        tx
    }
}
