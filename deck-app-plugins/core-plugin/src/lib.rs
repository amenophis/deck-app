use deck_app_plugin_sdk::action::Action;
use deck_app_plugin_sdk::plugin::{Plugin, PluginCommand};
use once_cell::unsync::Lazy;

static mut STATE: Lazy<State> = Lazy::new(|| State::new());

struct State
{
    counter: u8,
}

impl State
{
    pub fn new() -> Self
    {
        Self {
            counter: 0,
        }
    }
}

pub fn get_plugin() -> Plugin
{
    Plugin::new(
        "core",
        Box::new(|| unsafe {
            STATE.counter = 0;
        }),
        Box::new(|| {
        }),
    ).add_action(Action::new("set_page", Box::new(|_action, context| {
        // let config = &context.config;
        // let page: usize = config.get("page").unwrap().parse().unwrap();
        //
        // context.sender.send(PluginCommand::SetPage(context.clone(), page)).unwrap();

        unsafe {
            STATE.counter += 1;
            log::info!("CorePlugin change_page {}", STATE.counter);

            context.sender.send(PluginCommand::SetButtonText(context.clone(), "Coucou")).unwrap()
        }

    })))
}
