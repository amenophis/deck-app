use hidapi::HidApi;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use streamdeck::{ImageOptions};
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};
use tauri::async_runtime::spawn;
use tokio::time::sleep;
use crate::device::Device;

#[derive(serde::Deserialize)]
pub enum Command
{
    SetButtonImage(String, u8, String),
}

pub struct Server
{
    hid_api: Arc<Mutex<HidApi>>,
    devices: Arc<Mutex<HashMap<String, Device>>>,
    command_receiver: Arc<Mutex<Receiver<Command>>>,
    command_sender: Arc<Mutex<Sender<Command>>>,
}

impl Server
{
    pub fn new() -> Self
    {
        let hid_api = Arc::new(Mutex::new(HidApi::new().unwrap()));
        let devices = Arc::new(Mutex::new(HashMap::<String, Device>::new()));
        let (command_sender, command_receiver): (Sender<Command>, Receiver<Command>) = mpsc::channel(32);

        Self {
            hid_api,
            devices,
            command_receiver: Arc::new(Mutex::new(command_receiver)),
            command_sender: Arc::new(Mutex::new(command_sender)),
        }
    }

    pub fn get_devices(&self) -> Arc<Mutex<HashMap<String, Device>>>
    {
        self.devices.clone()
    }

    pub async fn execute_command(&self, c: Command)
    {
        self.command_sender.lock().await.send(c).await.unwrap();
    }

    pub fn start(&mut self)
    {
        self.streamdeck_watcher();
        self.command_handler();
    }

    fn streamdeck_watcher(&mut self) {
        let hid_api = self.hid_api.clone();
        let devices = self.devices.clone();

        // Streamdecks Watcher
        spawn(async move {
            loop {
                let mut attached_serials = Vec::new();

                // Look for new streamdecks
                {
                    let mut hid_api = hid_api.lock().await;
                    hid_api.refresh_devices().unwrap();

                    let attached_streamdecks = hid_api.device_list()
                        .filter(|streamdeck| streamdeck.vendor_id() == 0x0FD9);

                    for streamdeck in attached_streamdecks {
                        let serial = streamdeck.serial_number().unwrap().to_string();
                        let mut device_list = devices.lock().await;

                        attached_serials.push(serial.clone());

                        if let None = device_list.get(&serial) {
                            let device = Device::new(&hid_api, streamdeck.vendor_id(), streamdeck.product_id(), serial.clone());
                            device_list.insert(serial.clone(), device);

                            println!("Attached {}", serial);
                            // TODO: Dispatch attached event
                        }
                    };
                }

                // Look for suspended streamdecks
                // TODO later

                {
                    let mut devices = devices.lock().await;

                    // Remove unplugged streamdecks
                    let mut to_remove = Vec::new(); // TODO: Search how to optimize without an extra Vec ?
                    for serial in devices.keys() {
                        if !attached_serials.contains(&serial) {
                            to_remove.push(serial.clone());
                        }
                    }
                    for serial in to_remove {
                        devices.remove(&serial);

                        println!("Detached {}", serial);
                        // TODO: Dispatch attached event
                    }
                }

                sleep(Duration::from_secs(1)).await;
            }
        });
    }

    fn command_handler(&mut self)
    {
        let command_receiver = self.command_receiver.clone();
        let devices = self.devices.clone();

        spawn(async move {
            loop {
                while let Some(command) = command_receiver.lock().await.recv().await {
                    match command {
                        Command::SetButtonImage(serial, key, image) => {
                            let mut handles = devices.lock().await;
                            let device = handles.get_mut(&serial).unwrap();

                            device.streamdeck.set_button_file(key, image.as_str(), &ImageOptions::default()).expect("TODO: panic message");
                        }
                    }
                }
            }
        });
    }
}
