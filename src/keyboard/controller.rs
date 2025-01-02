use std::io;

use crate::{spotify_player::player::PlayerControl, types::spotify::SpotifyControlKind};
pub struct Controller {
    pub player: PlayerControl,
}

impl Controller {
    pub fn listen_for_key_input(&self) {
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).unwrap();
            self.handle_key_press(buffer);
        }
    }

    fn handle_key_press(&self, buffer: String) {
        println!("buffer: {:?}", buffer);
        match buffer {
            // column 1
            x if x.contains('3') => self
                .player
                .control_playback(SpotifyControlKind::RandomAlbum),
            x if x.contains('2') => self.player.restart_spotify(),
            x if x.contains('1') => self.player.control_playback(SpotifyControlKind::VolumeUp),
            x if x.contains('0') => self.player.control_playback(SpotifyControlKind::VolumeDown),
            // column 2
            x if x.contains('7') => self
                .player
                .control_playback(SpotifyControlKind::RandomArtist),
            x if x.contains('5') => self.player.control_playback(SpotifyControlKind::Shuffle),
            x if x.contains('4') => self.player.control_playback(SpotifyControlKind::Previous),
            // column 3
            x if x.contains('z') => self
                .player
                .control_playback(SpotifyControlKind::RandomPlaylist),
            x if x.contains('8') => self.player.control_playback(SpotifyControlKind::PlayPause),
            // column 4
            x if x.contains('e') => self
                .player
                .control_playback(SpotifyControlKind::RandomAnything),
            x if x.contains('ä') => self.player.control_playback(SpotifyControlKind::Next),
            _ => (),
        }
    }
}
