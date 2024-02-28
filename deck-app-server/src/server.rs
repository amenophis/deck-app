use std::collections::HashMap;
use std::{env, fs, thread};
use std::time::Duration;
use ::streamdeck::{StreamDeck};
use crossbeam::channel;
use hidapi::HidApi;
use rusttype::Font;
use streamdeck::{ImageOptions, TextOptions, TextPosition};
use deck_app_plugin_sdk::action::ActionContext;
use crate::config::Config;
use crate::device::Device;
use crate::display::Display;
use deck_app_plugin_sdk::plugin::{PluginCommand, PluginList};

const WAIT_TIME_MICROS: u64 = 100_000; // 100 ms

pub struct Server {
    config: Config,
    hid_api: HidApi,
    plugins: PluginList,
    connected_devices: HashMap<String, Device>,
    plugin_command_tx: channel::Sender<PluginCommand>,
    plugin_command_rx: channel::Receiver<PluginCommand>,
}

impl Server {
    pub fn new() -> Result<Self, ()>
    {
        let (plugin_command_tx, plugin_command_rx) = channel::unbounded::<PluginCommand>();

        Ok(Self {
            config: Self::load_config(),
            hid_api: Self::init_hid_api(),
            plugins: PluginList::new(),
            connected_devices: HashMap::new(),
            plugin_command_tx,
            plugin_command_rx,
        })
    }

    pub fn run(&mut self)
    {
        self.init_plugins();

        loop {
            self.refresh_devices();

            if !self.connected_devices.is_empty() {
                self.refresh_buttons_states();
                self.execute_plugins();
                self.handle_plugin_command();
                self.render();
            }

            thread::sleep(Duration::from_micros(WAIT_TIME_MICROS));
        }
    }

    fn init_plugins(&mut self)
    {
        let core = core_plugin::get_plugin();
        self.plugins.insert(core.id, core);

        let keyboard = keyboard_plugin::get_plugin();
        self.plugins.insert(keyboard.id, keyboard);
    }

    fn load_config() -> Config
    {
        let mut path = env::current_dir().unwrap();
        path.push("../../config.json");

        let config_file = fs::read_to_string(path).expect("Should have been able to read the file");
        let config: Config = serde_json::from_str(&config_file).expect("Unable to parse config");

        config
    }

    fn init_hid_api() -> HidApi
    {
        HidApi::new().unwrap()
    }

    fn refresh_devices(&mut self)
    {
        match self.hid_api.refresh_devices() {
            Ok(_) => {
                let mut attached_serials = Vec::new();

                // Search for attached devices and dispatch DeviceAttached command
                for device_info in self.hid_api.device_list().filter(|d| d.vendor_id() == 0x0FD9) {
                    let serial = device_info.serial_number().unwrap().to_string();
                    attached_serials.push(serial.clone());

                    if !self.connected_devices.contains_key(&serial) {
                        self.connected_devices.insert(serial.clone(), Device {
                            serial: serial.clone(),
                            vendor_id: device_info.vendor_id(),
                            product_id: device_info.product_id(),
                            current_page: 0,
                            button_states: Vec::new(),
                            display: Display::new(),
                            streamdeck: StreamDeck::connect_with_hid(&self.hid_api, device_info.vendor_id(), device_info.product_id(), Some(serial.clone())).expect("TODO: panic message")
                        });
                        log::info!("[{}] DeviceAttached", serial);
                    }
                }

                // Search for detached devices and dispatch DeviceDetached command
                let serials: Vec<String> = self.connected_devices.keys().cloned().collect();
                for serial in serials {
                    if !&attached_serials.contains(&serial) {
                        let _ = self.connected_devices.remove(&serial);
                        log::info!("[{}] DeviceDetached", serial);
                    }
                }
            }
            Err(e) => log::error!("An error occurred in watcher loop: {}", e)
        }
    }


    fn refresh_buttons_states(&mut self)
    {
        for (serial, device) in &mut self.connected_devices {
            let new_button_states = {
                let states = device.streamdeck.read_buttons(Some(Duration::from_millis(1))).unwrap_or_else(|_| Vec::new());
                if states.is_empty() {
                    continue;
                }

                states
            };

            for (key, new_button_state) in new_button_states.iter().enumerate() {
                let current_button_state: u8 = match device.button_states.get(key) {
                    None => {
                        let initial_value = 0;
                        device.button_states.push(initial_value);

                        initial_value
                    },
                    Some(state) => *state
                };

                let key_u8 = key.try_into().unwrap();

                if let Some(button) = &self.config.get_button(&serial, device.current_page, key_u8) {
                    let action_config = match (new_button_state, current_button_state) {
                        (1, 0) => &button.press,
                        (0, 1) => &button.release,
                        _ => &None
                    };

                    if let Some(action_config) = action_config {
                        let plugin = self.plugins.get_mut(action_config.plugin.as_str()).expect("TODO: panic message");
                        plugin.run_action(
                            action_config.action.as_str(),
                                ActionContext {
                                config: action_config.config.clone(),
                                key: key_u8,
                                serial: serial.clone(),
                                sender: self.plugin_command_tx.clone()
                            }
                        );
                    }
                }

                device.button_states[key] = *new_button_state;
            }
        }
    }

    fn execute_plugins(&mut self)
    {
        for (_, plugin) in &mut self.plugins.iter_mut() {
            let _ = plugin.run();
        }
    }

    fn handle_plugin_command(&mut self)
    {
        if let Ok(command) = self.plugin_command_rx.try_recv() {
            match command {
                PluginCommand::SetButtonImage(context) => {
                    log::info!("[{}] SetButtonImage - key: {}", context.serial, context.key);

                    let mut path = env::current_dir().unwrap();
                    path.push("../power.png");

                    let device = self.connected_devices.get_mut(&context.serial).expect("TODO: panic message");

                    device.streamdeck.set_button_file(context.key, path.display().to_string().as_str(), &ImageOptions::default()).expect("TODO: panic message");
                }
                PluginCommand::SetPage(context, page) => {
                    let device = self.connected_devices.get_mut(&context.serial).expect("TODO: panic message");
                    device.current_page = page;
                }
                PluginCommand::SetButtonText(context, text) => {
                    let device = self.connected_devices.get_mut(&context.serial).expect("TODO: panic message");

                    let font_data: &[u8] = include_bytes!("../COMIC.TTF");
                    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
                    
                    device.streamdeck.set_button_text(
                        context.key,
                        &font,
                        &TextPosition::Absolute { x: 0, y: 0 },
                        text,
                        &TextOptions::default()
                    ).unwrap();
                }
            }
        }
    }

    fn render(&mut self)
    {
        for (_, device) in &self.connected_devices {
            // let page_config = self.config.get_page(serial, device.current_page).expect("TODO: panic message");

            device.display.render();
            // log::info!("[{}] render", serial);
        }
    }
}

