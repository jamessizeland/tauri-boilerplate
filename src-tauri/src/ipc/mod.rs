//! Inter-process communications
//!
//! Recieve actions from the Front End and respond to them.

use super::utils::{Data, DataStore};
use crate::utils::get_unix_time;
use serde::{ser::StdError, Serialize};
use std::{sync::Mutex, thread};
use tauri::{async_runtime::Receiver, Manager}; // mutual exclusion wrapper

#[derive(Clone, Serialize, Debug)]
pub struct Payload {
    pub event: String,
    pub payload: String,
}

pub struct QueueHandler(pub Mutex<tauri::async_runtime::Sender<Payload>>);

/// Set up a separate thread handler to pass data via events from Tauri -> frontend
pub fn spawn_event_thread(
    app: &mut tauri::App,
    mut rx: Receiver<Payload>,
) -> Result<(), Box<(dyn StdError + 'static)>> {
    let window = app.get_window("main").unwrap();
    let _handle = thread::spawn(move || {
        println!("spawning a new thread to handle unprompted events from Rust to the UI");
        loop {
            let payload = rx.blocking_recv().unwrap();
            println!("{}, {}", payload.event, payload.payload);
            window.emit(&payload.event, payload.payload).unwrap();
        }
    });
    Ok(())
}

#[tauri::command]
/// Example of getting an event from frontend and returning a response
pub fn mod_state(message: &str, state: tauri::State<DataStore>) -> Data {
    let mut store = state.0.lock().expect("failed to unlock");
    println!("{}", message);
    store.message = message.to_owned(); // modify the message stored
    store.created_at = get_unix_time();
    store.clone() // deref a copy of the store to send to the frontend
}
