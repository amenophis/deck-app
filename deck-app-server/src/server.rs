use std::collections::HashMap;
use std::{env, fs, thread};
use std::time::Duration;
use ::streamdeck::StreamDeck;
use crossbeam::channel::{Receiver, Sender, unbounded};
use hidapi::HidApi;
use crate::server::config::Config;
use crate::server::plugin::{Plugin, PluginCommand};

mod plugin;
mod config;

#[derive(Clone)]
pub enum Command
{
    DeviceAttached(String),
    DeviceDetached(String),
    KeyPressed(String, u8),
    KeyReleased(String, u8),
}

pub struct Device
{
    pub serial: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub current_page: usize,
    pub button_states: Vec<u8>,
    pub streamdeck: StreamDeck,
}

pub struct Server {
    config: Config,
    plugins: Vec<Plugin>,
    hid_api: HidApi,
    connected_devices: HashMap<String, Device>,
    tx: Sender<Command>,
    rx: Receiver<Command>,
}

impl Server {
    pub fn new() -> Result<Self, ()>
    {
        env_logger::builder()
            .format_timestamp_micros()
            .init();

        let config = load_config();

        let (tx, rx) = unbounded::<Command>();

        Ok(Self {
            config,
            plugins: Vec::new(),
            hid_api: HidApi::new().unwrap(),
            connected_devices: HashMap::new(),
            tx,
            rx
        })
    }


    pub fn run(&mut self)
    {
        self.load_plugins();

        loop {
            self.refresh_devices();
            self.refresh_buttons();
            self.handle_command();

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn load_plugins(&mut self)
    {
        let plugins_dir = fs::read_dir(self.config.plugins_path.clone()).expect("Plugin folder is missing");

        for plugin_dir in plugins_dir {
            let dir = plugin_dir.expect("");
            if !dir.file_type().expect("").is_dir() {
                continue;
            }

            self.plugins.push(
                Plugin::new(
                    self.config.plugins_path.clone(),
                    dir.file_name().to_str().expect("").to_string()
                )
            );
        }
    }

    fn refresh_devices(&mut self)
    {
        match self.hid_api.refresh_devices() {
            Ok(_) => {
                let mut attached_serials = Vec::new();

                // Adding not existing devices
                for d in self.hid_api.device_list() {
                    if d.vendor_id() != 0x0FD9 {
                        continue;
                    }

                    let serial = d.serial_number().unwrap().to_string();
                    attached_serials.push(serial.clone());

                    if !self.connected_devices.contains_key(&serial) {
                        self.connected_devices.insert(serial.clone(), Device {
                            serial: serial.clone(),
                            vendor_id: d.vendor_id(),
                            product_id: d.product_id(),
                            current_page: 0,
                            button_states: Vec::new(),
                            streamdeck: StreamDeck::connect(d.vendor_id(), d.product_id(), Some(serial.clone())).expect("TODO: panic message")
                        });

                        let _ = self.tx.send(Command::DeviceAttached(serial.clone()));
                    }
                }

                let mut devices_to_remove = Vec::new();

                for serial in self.connected_devices.keys() {
                    if !&attached_serials.contains(serial) {
                        devices_to_remove.push(serial.clone());
                    }
                }

                for serial in devices_to_remove {
                    self.connected_devices.remove(&serial);
                    let _ = self.tx.send(Command::DeviceDetached(serial.clone()));
                }



            }
            Err(e) => log::error!("An error occurred in watcher loop: {}", e)
        }
    }


    fn refresh_buttons(&mut self)
    {
        for (serial, device) in &mut self.connected_devices {
            let new_button_states = {
                let states = device.streamdeck.read_buttons(None).unwrap_or_else(|_| Vec::new());
                if states.len() == 0 {
                    continue;
                }

                states
            };

            let mut key = 0;

            for new_button_state in new_button_states {
                let current_button_state: u8 = match device.button_states.get(key) {
                    None => {
                        let initial_value = 0;
                        device.button_states.push(initial_value);

                        initial_value
                    },
                    Some(state) => *state
                };

                match (new_button_state, current_button_state) {
                    (1, 0) => self.tx.send(Command::KeyPressed(serial.clone(), key as u8)).expect("TODO: panic message"),
                    (0, 1) => self.tx.send(Command::KeyReleased(serial.clone(), key as u8)).expect("TODO: panic message"),
                    _ => ()
                };

                device.button_states[key] = new_button_state;

                key = key + 1;
            }
        }
    }

    fn handle_command(&mut self)
    {
        if let Ok(command) = self.rx.try_recv() {
            match command {
                Command::DeviceAttached(serial) => {
                    log::info!("[{:?}] DeviceAttached", serial);

                    for plugin in &self.plugins {
                        plugin.execute(PluginCommand::DeviceAttached(serial.clone()));
                    }
                }
                Command::DeviceDetached(serial) => {
                    log::info!("[{:?}] DeviceDetached", serial);

                    for plugin in &self.plugins {
                        plugin.execute(PluginCommand::DeviceDetached(serial.clone()));
                    }
                }
                Command::KeyPressed(serial, key) => {
                    log::info!("[{:?}] KeyPressed - key: {}", serial, key);

                    let device = self.connected_devices.get(&serial).expect("TODO: panic message");
                    if let Some(button) = &self.config.get_button(&serial, device.current_page, key) {
                        if let Some(action) = &button.press {
                            for plugin in &self.plugins {
                                if plugin.id == action.namespace {
                                    plugin.execute(PluginCommand::KeyPressed(serial.clone(), key));
                                }
                            }
                        }
                    }
                }
                Command::KeyReleased(serial, key) => {
                    log::info!("[{:?}] KeyReleased - key: {}", serial, key);

                    let device = self.connected_devices.get(&serial).expect("TODO: panic message");
                    if let Some(button) = &self.config.get_button(&serial, device.current_page, key) {
                        if let Some(action) = &button.release {
                            for plugin in &self.plugins {
                                if plugin.id == action.namespace {
                                    plugin.execute(PluginCommand::KeyReleased(serial.clone(), key));
                                }
                            }
                        }
                    }
                }
                // Command::UpdateButton(s, key, _image) => {
                //     info!("UpdateButton {:?}", key);
                //
                //     let mut path = env::current_dir().unwrap();
                //     path.push("../power.png");
                //
                //     let streamdeck_arc = connected_streamdecks.get_mut(&s.serial.clone()).expect("TODO: panic message");
                //     let mut streamdeck = streamdeck_arc.lock().await;
                //
                //     streamdeck.set_button_file(key, path.display().to_string().as_str(), &ImageOptions::default()).expect("TODO: panic message");
                // }
            }
        }
    }
}

fn load_config() -> Config
{
    let mut path = env::current_dir().unwrap();
    path.push("../../config.json");

    let config_file = fs::read_to_string(path).expect("Should have been able to read the file");
    let config: Config = serde_json::from_str(&config_file).expect("Unable to parse config");

    config
}
