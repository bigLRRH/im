use rusqlite::{params, Connection, Result};
use crate::types::conversation::Conversation;

pub fn create_conversation(conn: &Connection, types: i32, name: &str) -> Result<Conversation> {
    conn.execute(
        "INSERT INTO conversations (types, name) VALUES (?1, ?2)",
        params![types, name],
    )?;

    let id = conn.last_insert_rowid();
    Ok(Conversation::new(id, types, name.to_string()))
}

pub fn get_conversation(conn: &Connection, id: i64) -> Result<Conversation> {
    let mut stmt = conn.prepare("SELECT id, types, name FROM conversations WHERE id = ?1")?;
    let conversation_iter = stmt.query_map(params![id], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            types: row.get(1)?,
            name: row.get(2)?,
        })
    })?;

    for conversation in conversation_iter {
        return conversation;
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}
