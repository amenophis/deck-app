use streamdeck::StreamDeck;
use crate::display::Display;

pub struct Device
{
    pub serial: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub current_page: usize,
    pub button_states: Vec<u8>,
    pub display: Display,
    pub streamdeck: StreamDeck,
}
