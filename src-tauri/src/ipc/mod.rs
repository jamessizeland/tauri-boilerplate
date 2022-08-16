//! Inter-process communications
//!
//! Recieve actions from the Front End and respond to them.

use crate::utils::get_unix_time;

use super::utils::{Data, DataStore};

#[tauri::command]
/// Example of getting an event from frontend and returning a response
pub fn mod_state(message: &str, state: tauri::State<DataStore>) -> Data {
    let mut store = state.0.lock().expect("failed to unlock");
    println!("{}", message);
    store.message = message.to_owned(); // modify the message stored
    store.created_at = get_unix_time();
    store.clone() // deref a copy of the store to send to the frontend
}
