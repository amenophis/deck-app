use std::collections::HashMap;
use extism::{Manifest, Wasm};
use crate::server::Command;

pub enum PluginCommand
{
    DeviceAttached(String),
    DeviceDetached(String),
    KeyPressed(String, u8),
    KeyReleased(String, u8),
}

pub struct Plugin {
    pub id: String,
    wasm: extism::Plugin,
    pub actions: HashMap<String, Action>,
}

impl Plugin {
    pub fn new(path: String, id: String) -> Self
    {
        let action1_id = "action1".to_string();
        let action1_script = "console.log(\"action1\")".to_string();
        let action1 = Action::new(action1_id.clone(), action1_script);

        let path = format!("{}/{}/dist/plugin.wasm", path, id);

        let url = Wasm::file(path);
        let manifest = Manifest::new([url]);
        let plugin = extism::Plugin::new(&manifest, [], true).unwrap();

        let mut actions = HashMap::new();
        actions.insert(action1_id, action1);

        Self {
            id,
            wasm: plugin,
            actions,
        }
    }

    pub fn execute(&self, command: PluginCommand)
    {
        match command {
            PluginCommand::DeviceAttached(serial) => {
                log::info!("[{:?}] DeviceAttached", serial);
            }
            PluginCommand::DeviceDetached(serial) => {
                log::info!("[{:?}] DeviceDetached", serial);
            }
            PluginCommand::KeyPressed(serial, key) => {
                log::info!("[{:?}] KeyPressed - key: {}", serial, key);
            }
            PluginCommand::KeyReleased(serial, key) => {
                log::info!("[{:?}] KeyReleased - key: {}", serial, key);
            }
        }
    }
}

pub struct Action {
    pub id: String,
    pub script: String,
}

impl Action {
    pub fn new(id: String, script: String) -> Self
    {
        Self {
            id,
            script,
        }
    }

    pub async fn execute(&self, plugin: &String)
    {
        log::info!("Executing {}.{}", plugin, self.id);

        // run_js().await;
    }
}
