use p2p::{messaging::P2PCommand, P2PNode};
use tauri::{async_runtime, Emitter, Manager};
use tokio::sync::mpsc::UnboundedSender;

mod commands;
mod dao;
mod p2p;
mod services;
mod types;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub command_sender: UnboundedSender<P2PCommand>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let listen_addr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
            let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel();
            let p2p_node = async_runtime::block_on(async {
                P2PNode::new(listen_addr, event_sender)
                    .await
                    .expect("初始化 P2PNode 失败")
            });

            let command_sender = p2p_node.command_sender();
            app.manage(AppState { command_sender });

            // 启动事件转发任务（将 P2P 事件转发到前端）
            let app_handle = app.handle().clone();
            async_runtime::spawn(async move {
                while let Some(event) = event_receiver.recv().await {
                    app_handle
                        .emit("p2p-event", event) // 使用统一事件名
                        .unwrap_or_else(|e| eprintln!("事件发送失败: {}", e));
                }
            });

            // 启动 P2P 主循环（转移所有权）
            async_runtime::spawn(async move {
                p2p_node.run().await.expect("P2PNode 运行失败");
            });

            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
