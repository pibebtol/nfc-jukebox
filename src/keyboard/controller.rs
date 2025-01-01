use crossterm::event::{read, Event, KeyEvent};

use crate::spotify_player::player::PlayerControl;
pub struct Controller {
    player: PlayerControl,
}

impl Controller {
    pub fn new(player: PlayerControl) -> Controller {
        Controller { player }
    }

    pub fn listen_for_key_input(&self) {
        loop {
            match read().unwrap() {
                Event::Key(event) => self.handle_key_event(event),
                _ => (),
            }
        }
    }

    fn handle_key_event(&self, event: KeyEvent) {
        match event.code {
            crossterm::event::KeyCode::Char(code) => self.handle_key_press(code),
            _ => (),
        }
    }

    fn handle_key_press(&self, code: char) {
        match code {
            '0' => self.player.volume_down(),
            '1' => self.player.volume_up(),
            _ => (),
        }
    }
}
