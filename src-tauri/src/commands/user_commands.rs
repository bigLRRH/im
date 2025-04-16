use crate::types::user::User;
use tauri::command;

#[command]
fn insert_user(user: User) -> Result<String, String> {
    // Here you would typically insert the user into a database or perform some action.
    // For demonstration, we'll just return a success message.
    Ok(format!("User {} inserted successfully!", user.username))
}

#[command]
fn delete_user(user_id: String) -> Result<String, String> {
    // Here you would typically delete the user from a database or perform some action.
    // For demonstration, we'll just return a success message.
    Ok(format!("User with ID {} deleted successfully!", user_id))
}

#[command]
fn select_user(user_id: String) -> Result<String, String> {
    // Here you would typically select the user from a database or perform some action.
    // For demonstration, we'll just return a success message.
    Ok(format!("User with ID {} selected successfully!", user_id))
}

#[command]
fn update_user(user: User) -> Result<String, String> {
    // Here you would typically update the user in a database or perform some action.
    // For demonstration, we'll just return a success message.
    Ok(format!("User {} updated successfully!", user.username))
}
