use tokio::sync::mpsc::UnboundedSender;

use crate::p2p::messaging::P2PCommand;

pub fn send_p2p_event(sender: &UnboundedSender<P2PCommand>, command: String) -> Result<(), String> {
    let command = match serde_json::from_str::<P2PCommand>(&command) {
        Ok(cmd) => cmd,
        Err(e) => return Err(format!("Failed to parse command: {}", e)),
    };
    sender
        .send(command)
        .map_err(|e| format!("Failed to send event: {}", e))
}
