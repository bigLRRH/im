import { invoke } from '@tauri-apps/api/core';
import { P2PCommand } from '@/types/p2p';

/**
 * 调用后端 handle_p2p_event，发送 P2P 命令
 * @param command P2PCommand 类型，包含订阅或发布信息
 */
export async function invokeP2PCommand(command: P2PCommand): Promise<void> {
  try {
    await invoke('handle_p2p_event', { command });
    console.log('✅ P2P 命令已发送:', command);
  } catch (error) {
    console.error('❌ 发送 P2P 命令失败:', error);
  }
}
