extern crate hidapi;

use std::env;
use tauri::State;
use crate::server::{Command, Server};

#[derive(serde::Serialize)]
pub struct DeviceResponse
{
    serial: String,
    version: String,
}

#[tauri::command]
pub async fn get_decks(server: State<'_, Server>) -> Result<Vec<DeviceResponse>, String>
{
    let mut devices = Vec::new();

    for (serial, device) in server.get_devices().lock().await.iter_mut() {
        devices.push(DeviceResponse {
            serial: serial.to_string(),
            version: device.streamdeck.version().unwrap(),
        })
    }

    Ok(devices)
}

#[tauri::command]
pub async fn set_button_image(server: State<'_, Server>, serial: String, key: u8) -> Result<(), String>
{
    let mut path = env::current_dir().unwrap();
    path.push("../power.png");

    dbg!(path.display());

    let command = Command::SetButtonImage(serial, key, path.display().to_string());

    server.execute_command(command).await;

    Ok(())
}
