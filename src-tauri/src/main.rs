#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ipc;
mod utils;

use tauri::Manager;
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // listen to the `event-name` (emitted on any window)
            let id = app.listen_global("event-name", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });
            // unlisten to the event using the `id` returned on the `listen_global` function
            // an `once_global` API is also exposed on the `App` struct
            app.unlisten(id);

            // emit the `event-name` event to all webview windows on the frontend
            app.emit_all(
                "event-name",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
            Ok(())
        })
        .manage(utils::DataStore(Default::default()))
        .invoke_handler(tauri::generate_handler![
            // list all callable actions here
            ipc::mod_state
        ])
        .run(tauri::generate_context!()) // TODO run yarn build to fix this error
        .expect("error while running tauri application");
}
