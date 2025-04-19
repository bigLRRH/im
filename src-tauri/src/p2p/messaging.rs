use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum P2PCommand {
    SubscribeTopic(String),
    PublishMessage { topic: String, message: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum P2PEvent {
    MessageReceived { topic: String, data: String },
    PeerDiscovered(String),
}
