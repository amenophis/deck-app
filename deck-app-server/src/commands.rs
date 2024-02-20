// use std::env;
// use eyre::Context;
// use tauri::State;
// use crate::server::{Command, Server};
//
// #[derive(serde::Serialize)]
// pub struct DeviceResponse
// {
//     serial: String,
//     version: String,
// }
//
// #[tauri::command]
// pub async fn get_decks(server: State<'_, Server>) -> Result<Vec<DeviceResponse>, String>
// {
//     let mut devices = Vec::new();
//
//     for (serial, device) in server.get_devices().lock().await.iter_mut() {
//         let version = device.streamdeck.lock().await.version().wrap_err("Unable to get streamdeck version");
//         match version {
//             Ok(version) => {
//                 devices.push(DeviceResponse {
//                     serial: serial.to_string(),
//                     version
//                 })
//             }
//             Err(e) => println!("{}", e)
//         }
//     }
//
//     Ok(devices)
// }
//
// #[tauri::command]
// pub async fn set_button_image(server: State<'_, Server>, serial: String, key: u8) -> Result<(), String>
// {
//     let path = env::current_dir().wrap_err("Unable to get current_dir");
//     match path {
//         Ok(mut path) => {
//             path.push("../power.png");
//
//             let command = Command::SetButtonImage(serial, key, path.display().to_string());
//
//             let _ = server.execute_command(command).await;
//
//             Ok(())
//         }
//         Err(e) => {
//             Err(e.to_string())
//         }
//     }
//
// }
