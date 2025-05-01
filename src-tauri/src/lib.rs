use commands::p2p_commands::handle_p2p_event;
use p2p::P2PNode;
use tauri::{async_runtime, Emitter, Manager};

mod commands;
mod p2p;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // * 可放入service层
            let listen_addr = "/ip4/0.0.0.0/tcp/0".parse().unwrap();
            let (event_sender, mut event_receiver) = tokio::sync::mpsc::unbounded_channel();
            let p2p_node = async_runtime::block_on(async {
                P2PNode::new(listen_addr, event_sender)
                    .await
                    .expect("初始化 P2PNode 失败")
            });

            let command_sender = p2p_node.command_sender();
            app.manage(command_sender);

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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_p2p_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
