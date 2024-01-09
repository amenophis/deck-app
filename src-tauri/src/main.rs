// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
mod server;
mod device;

use tauri::Manager;
use server::Server;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::get_decks, commands::set_button_image])
        .setup(move |app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            let server = Server::new();
            match server {
                Ok(mut server) => {
                    server.start();
                    app.manage(server);
                }
                Err(e) => println!("Unable to start server: {}", e)
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

