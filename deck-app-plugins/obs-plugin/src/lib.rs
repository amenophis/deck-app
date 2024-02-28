use deck_app_plugin_sdk::plugin::Plugin;
use deck_app_plugin_sdk::action::ActionList;
use crate::change_scene::ChangeSceneAction;

mod change_scene;

#[derive(Default)]
pub struct OBSPlugin
{
    plugin: Plugin,
}

impl OBSPlugin
{
    pub fn new() -> OBSPlugin
    {
        let mut actions = ActionList::new();
        actions.insert(ChangeSceneAction::get_id(), Box::<ChangeSceneAction>::default());
        Self {
            actions
        }
    }
}

impl Plugin for OBSPlugin
{
    fn get_id(&self) -> &'static str
    {
        "keyboard-plugin"
    }

    fn get_actions(&self) -> &ActionList {
        &self.actions
    }

    fn run(&mut self) {
        log::info!("OBSPlugin run");
    }
}
