use enigo::{Enigo, KeyboardControllable};
use lazy_static::lazy_static;
use deck_app_plugin_sdk::action::Action;
use deck_app_plugin_sdk::plugin::{Plugin, PluginCommand};

lazy_static! {
    static ref STATE: State = State::new();
}

struct State
{
}

impl State
{
    pub fn new() -> Self
    {
        Self {
        }
    }
}

pub fn get_plugin() -> Plugin
{
    Plugin::new(
        "keyboard",
        Box::new(|| {
        }),
        Box::new(|| {
        })
    ).add_action(Action::new("type", Box::new(|_, context| {
        let config = &context.config;

        let mut enigo = Enigo::default();
        enigo.set_delay(100);

        log::info!("Type action");
        enigo.key_sequence(config.get("text").unwrap_or(&"Action config text is missing".to_string()));
        log::info!("Type action done");

        context.sender.send(PluginCommand::SetButtonImage(context.clone())).unwrap();
    })))
}
