use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub version: u8,
    pub streamdecks: HashMap<String, Streamdeck>,
}

impl Config
{
    pub fn get_streamdeck(&self, serial: &str) -> Option<&Streamdeck>
    {
        self.streamdecks.get(serial)
    }

    pub fn get_page(&self, serial: &str, page: usize,) -> Option<&Page>
    {
        let streamdeck = self.get_streamdeck(serial).expect("Unable to get streamdeck");
        if page > (streamdeck.pages.len() - 1) {
            return None;
        }
        
        Some(&streamdeck.pages[page])
    }
    
    pub fn get_button(&self, serial: &str, page: usize, key: u8) -> Option<&Button>
    {
        let page = self.get_page(serial, page).expect("Unable to get streamdeck");

        return page.buttons.iter().find(|b| b.key == key)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Streamdeck {
    pub pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Page {
    pub buttons: Vec<Button>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Button {
    pub key: u8,
    pub press: Option<Action>,
    pub release: Option<Action>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Action
{
    pub plugin: String,
    pub action: String,
    pub config: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionConfig
{
    pub key: String,
    pub value: String,
}
