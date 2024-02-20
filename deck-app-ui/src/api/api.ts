import {invoke} from "@tauri-apps/api/tauri";

type Deck = {
  vid: Number,
  pid: Number,
  serial: string,
}

export async function getDecks() : Promise<Deck[]> {
  return await invoke('get_decks');
}

export async function setButtonImage(serial: string, key: Number) : Promise<void> {
  return await invoke('set_button_image', {
    serial,
    key,
  });
}
