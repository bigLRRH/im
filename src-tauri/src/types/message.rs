#[derive(Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub sender_id: i64,
    pub conversation_id: i64,
    pub created_at: String, // 使用字符串表示日期时间，可以进一步优化为 NaiveDateTime
    pub message_type: i32,  // 0: 文本消息, 1: 图片消息, 2: 语音消息
    pub content: String,
}

impl Message {
    pub fn new(
        id: i64,
        sender_id: i64,
        conversation_id: i64,
        created_at: String,
        message_type: i32,
        content: String,
    ) -> Self {
        Message {
            id,
            sender_id,
            conversation_id,
            created_at,
            message_type,
            content,
        }
    }
}
