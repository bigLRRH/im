// commands/p2p_commands.rs
use tauri::State;
use tokio::sync::mpsc::UnboundedSender;
use crate::p2p::messaging::P2PCommand;

#[tauri::command]
pub fn handle_p2p_event(
    sender: State<'_, UnboundedSender<P2PCommand>>,
    command: P2PCommand,
) {
    sender.send(command).unwrap_or_else(|e| {
        eprintln!("❌ 发送 P2P 命令失败: {}", e);
    });
}
