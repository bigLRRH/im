#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: i64,
    pub types: i32, // 0: 单人会话, 1: 群组会话
    pub name: String,
}

impl Conversation {
    pub fn new(id: i64, types: i32, name: String) -> Self {
        Conversation { id, types, name }
    }
}
