// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate hidapi;

use crate::server::Server;

mod server;
mod commands;
// use tauri::Manager;

fn main() {
    env_logger::builder()
        .format_timestamp_micros()
        .init();

    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![commands::get_decks, commands::set_button_image])
        .setup(move |_| {
            tauri::async_runtime::spawn(async move {
                let mut server = Server::new().unwrap();
                server.run();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

