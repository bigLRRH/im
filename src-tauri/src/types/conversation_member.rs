#[derive(Debug, Clone)]
pub struct ConversationMember {
    pub conversation_id: i64,
    pub user_id: i64,
    pub role: i32, // 0: 普通成员, 1: 管理员
}

impl ConversationMember {
    pub fn new(conversation_id: i64, user_id: i64, role: i32) -> Self {
        ConversationMember {
            conversation_id,
            user_id,
            role,
        }
    }
}
