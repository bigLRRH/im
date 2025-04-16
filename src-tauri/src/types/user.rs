#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

impl User {
    pub fn new(id: i64, username: String) -> Self {
        User { id, username }
    }
}
