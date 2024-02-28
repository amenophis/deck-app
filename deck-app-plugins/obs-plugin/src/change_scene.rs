use deck_app_plugin_sdk::plugin::PluginCommand;
use deck_app_plugin_sdk::action::{Action, ActionContext};

#[derive(Default)]
pub struct ChangeSceneAction
{
}

impl ChangeSceneAction
{
    pub fn get_id() -> &'static str
    {
        "change_scene"
    }
}

impl Action for ChangeSceneAction
{
    fn execute(&self, context: ActionContext)
    {
        let _config = &context.config;

        context.sender.send(PluginCommand::SetButtonImage(context.clone())).unwrap();
    }
}
