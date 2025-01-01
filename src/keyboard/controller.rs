use crossterm::event::{read, Event, KeyCode, KeyEvent};

use crate::{spotify_player::player::PlayerControl, types::spotify::SpotifyControlKind};
pub struct Controller {
    player: PlayerControl,
}

impl Controller {
    pub fn new(player: PlayerControl) -> Controller {
        Controller { player }
    }

    pub fn listen_for_key_input(&self) {
        loop {
            if let Event::Key(event) = read().unwrap() {
                self.handle_key_event(event)
            }
        }
    }

    fn handle_key_event(&self, event: KeyEvent) {
        if let KeyCode::Char(code) = event.code {
            self.handle_key_press(code)
        }
    }

    fn handle_key_press(&self, code: char) {
        match code {
            '0' => self.player.control_playback(SpotifyControlKind::VolumeDown),
            '1' => self.player.control_playback(SpotifyControlKind::VolumeUp),
            '3' => self.player.control_playback(SpotifyControlKind::RandomAlbum),
            '4' => self.player.control_playback(SpotifyControlKind::Previous),
            '5' => self.player.control_playback(SpotifyControlKind::Shuffle),
            '7' => self.player.control_playback(SpotifyControlKind::RandomArtist),
            '8' => self.player.control_playback(SpotifyControlKind::PlayPause),
            'z' => self.player.control_playback(SpotifyControlKind::RandomPlaylist),
            'ä' => self.player.control_playback(SpotifyControlKind::Next),
            'e' => self.player.control_playback(SpotifyControlKind::RandomAnything),
            _ => (),
        }
    }
}
