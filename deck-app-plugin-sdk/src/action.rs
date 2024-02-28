use std::collections::HashMap;
use crossbeam::channel::Sender;
use crate::plugin::PluginCommand;

pub type ActionList = HashMap<&'static str, Action>;

type ActionExecuteFn = Box<dyn Fn(&Action, ActionContext) + 'static>;

pub struct Action
{
    pub id: &'static str,
    execute_fn: ActionExecuteFn,
}

impl Action
{
    pub fn new(id: &'static str, execute_fn: ActionExecuteFn) -> Self
    {
        Self {
            id,
            execute_fn,
        }
    }

    pub fn execute(&self, context: ActionContext)
    {
        (self.execute_fn)(self, context);
    }
}

#[derive(Clone)]
pub struct ActionContext
{
    pub serial: String,
    pub key: u8,
    pub config: HashMap<String, String>,
    pub sender: Sender<PluginCommand>,
}
