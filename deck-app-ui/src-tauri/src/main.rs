// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use deck_app_server::server::Server;

fn main() {
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![commands::get_decks, commands::set_button_image])
        .setup(move |_| {
            env_logger::builder()
                .format_timestamp_micros()
                .init();

            tauri::async_runtime::spawn(async move {
                let mut server = Server::new().unwrap();
                server.run();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

