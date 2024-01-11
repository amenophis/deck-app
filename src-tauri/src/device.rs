use std::sync::Arc;
use std::time::Duration;
use eyre::Error;
use hidapi::HidApi;
use streamdeck::StreamDeck;
use tauri::async_runtime::spawn;
use tokio::sync::Mutex;

pub struct Device {
    pub streamdeck: Arc<Mutex<StreamDeck>>,
}

pub enum Event {
    KeyPressed(usize),
    KeyReleased(usize),
}

impl Device {
    pub fn try_new(hid_api: &HidApi, vid: u16, pid: u16, serial: String) -> Result<Self, Error>
    {
        Ok(Self {
            streamdeck: Arc::new(Mutex::new(StreamDeck::connect_with_hid(hid_api, vid, pid, Some(serial))?)),
        })
    }

    pub fn start_button_watcher(&self, callback : fn(key: Event))
    {
        let streamdeck = self.streamdeck.clone();

        spawn(async move {
            let mut button_states = Vec::new();
            loop {
                let mut streamdeck = streamdeck.lock().await;

                match streamdeck.read_buttons(Some(Duration::from_millis(100))) {
                    Ok(new_button_states ) => {
                        let mut i = 0;
                        for new_button_state in new_button_states {
                            if let Some(current_button_state) = button_states.get(i) {
                                if new_button_state == 1 && *current_button_state == 0 {
                                    callback(Event::KeyPressed(i));
                                    println!("KeyPressed {i}");
                                } else if new_button_state == 0 && *current_button_state == 1 {
                                    callback(Event::KeyReleased(i));
                                    println!("KeyReleased {i}");
                                }
                                button_states[i] = new_button_state;
                            } else {
                                button_states.push(new_button_state);
                            }

                            i = i + 1;
                        }
                    }
                    Err(_) => {}
                }
            }
        });
    }
}
