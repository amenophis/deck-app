use std::collections::HashMap;
use crate::action::{Action, ActionContext, ActionList};

pub type PluginList = HashMap<&'static str, Plugin>;

type PluginRunFn = Box<dyn Fn() + 'static>;
type PluginInitFn = Box<dyn Fn() + 'static>;

#[derive(Clone)]
pub enum PluginCommand
{
    SetButtonImage(ActionContext),
    SetButtonText(ActionContext, &'static str),
    SetPage(ActionContext, usize),
}

pub struct Plugin
{
    pub id: &'static str,
    run_fn: PluginRunFn,
    actions: ActionList,
}

impl Plugin
{
    pub fn new(id: &'static str, init_fn: PluginInitFn, run_fn: PluginRunFn) -> Plugin
    {
        init_fn();

        Self {
            id,
            run_fn,
            actions: ActionList::new(),
        }
    }

    pub fn add_action(mut self, action: Action) -> Self
    {
        self.actions.insert(action.id, action);
        self
    }

    pub fn get_action(&self, id: &str) -> Option<&Action>
    {
        self.actions.get(id)
    }

    pub fn run_action(&self, id: &str, context: ActionContext)
    {
        self.actions.get(id).unwrap().execute(context);
    }

    pub fn run(&self)
    {
        (self.run_fn)();
    }
}
