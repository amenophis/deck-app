use eyre::Error;
use hidapi::HidApi;
use streamdeck::StreamDeck;

pub struct Device {
    pub streamdeck: StreamDeck,
}

impl Device {
    pub fn new(hid_api: &HidApi, vid: u16, pid: u16, serial: String) -> Result<Self, Error>
    {
        Ok(Self {
            streamdeck: StreamDeck::connect_with_hid(hid_api, vid, pid, Some(serial))?
        })
    }
}
