use std::sync::Mutex;

use tauri::State;
use tokio::sync::mpsc::UnboundedSender;

use crate::{p2p::messaging::P2PCommand, services::p2p_service};

#[tauri::command]
pub fn send_p2p_event(sender: State<'_, Mutex<UnboundedSender<P2PCommand>>>, command: String) {
    let sender = sender.lock().unwrap();
    p2p_service::send_p2p_event(&sender, command).unwrap_or_else(|e| {
        // log::error!("Failed to send event: {}", e);
        println!("Failed to send event: {}", e);
    });
}
