use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum P2PCommand {
    SubscribeTopic { topic: String },
    PublishMessage { topic: String, message: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum P2PEvent {
    MessageReceived { topic: String, data: String },
    PeerDiscovered { peer_id: String, multiaddr: String },
}
