use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: u8,
    pub plugins_path: String,
    pub streamdecks: HashMap<String, Streamdeck>,
}

impl Config
{
    pub fn get_button(&self, serial: String, page: usize, key: u8) -> Option<&Button>
    {
        let streamdeck = self.streamdecks.get(&serial).expect("Unable to get streamdeck");
        if page > (streamdeck.pages.len() - 1) {
            return None;
        }
        let page = &streamdeck.pages[page];

        for b in &page.buttons {
            if b.key == key {
               return Some(&b);
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Streamdeck {
    pub pages: Vec<Page>,
}

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub buttons: Vec<Button>,
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    pub key: u8,
    pub press: Option<Action>,
    pub release: Option<Action>,
}
#[derive(Serialize, Deserialize)]
pub struct Action
{
    pub namespace: String,
    pub id: String,
    // pub config: String,
}
