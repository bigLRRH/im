use rusqlite::{params, Connection, Result};
use crate::types::user::User;

pub fn create_user(conn: &Connection, username: &str) -> Result<User> {
    conn.execute(
        "INSERT INTO users (username) VALUES (?1)",
        params![username],
    )?;

    let id = conn.last_insert_rowid();
    Ok(User::new(id, username.to_string()))
}

pub fn get_user(conn: &Connection, id: i64) -> Result<User> {
    let mut stmt = conn.prepare("SELECT id, username FROM users WHERE id = ?1")?;
    let user_iter = stmt.query_map(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
        })
    })?;

    for user in user_iter {
        return user;
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}
