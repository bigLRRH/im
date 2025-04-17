// src/p2p/messaging.rs
#[derive(Debug, Clone)]
pub enum P2PMessage {
    ChatMessage { from: String, content: String },
}

#[derive(Debug, Clone)]
pub enum P2PCommand {
    SendMessage(String),
}
