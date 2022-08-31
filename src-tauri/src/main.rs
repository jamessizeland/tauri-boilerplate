#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ipc;
mod utils;

use ipc::{spawn_event_thread, Payload};
use std::sync::Mutex;
use tauri::async_runtime::channel;

fn main() {
    let (tx, rx) = channel::<Payload>(5);
    tauri::Builder::default()
        .setup(|app| spawn_event_thread(app, rx))
        .manage(utils::DataStore(Default::default()))
        .manage(ipc::QueueHandler(Mutex::new(tx)))
        .invoke_handler(tauri::generate_handler![
            // list all callable actions here
            ipc::mod_state
        ])
        .run(tauri::generate_context!()) // TODO run yarn build to fix this error
        .expect("error while running tauri application");
}
