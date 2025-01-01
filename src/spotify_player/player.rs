use std::process::Command;

use crate::{data_control::file_reader::FileDataProvider, types::{data_provider::DataProvider, spotify}};

#[derive(Clone)]
pub struct PlayerControl {
    pub data: FileDataProvider,
}

impl PlayerControl {
    pub fn play(&self, nfc_id: &str) {
        println!("{nfc_id}");
        let nfc_mappings = self.data.get_nfc_mappings();
        let nfc_index = nfc_mappings.iter().find(|&mapping| mapping.nfc_id == nfc_id).unwrap().index; 

        let spotify_mappings = self.data.get_spotify_mappings();
        let spotify_mapping = spotify_mappings.iter().find(|&mapping| mapping.nfc_index == nfc_index).unwrap();

        match spotify_mapping.spotify_kind {
            spotify::SpotifyKind::Album => self.play_album(&spotify_mapping.spotify_id),
            spotify::SpotifyKind::Artist => self.play_artist(&spotify_mapping.spotify_id),
            spotify::SpotifyKind::Playlist => self.play_playlist(&spotify_mapping.spotify_id),
            _ => (),
        }

    }

    pub fn volume_up(&self) {
        let vol = Self::get_current_volume() + 5;
        Self::set_volume(vol);
    }

    pub fn volume_down(&self) {
        let vol = Self::get_current_volume();
        if vol < 5 {
            Self::set_volume(0);
        } else {
            Self::set_volume(vol - 5);
        }
    }

    fn play_album(&self, spotify_id: &String) {
        self.play_context(spotify_id, "album".to_string());
    }

    fn play_artist(&self, spotify_id: &String) {
        self.play_context(spotify_id, "artist".to_string());
    }

    fn play_playlist(&self, spotify_id: &String) {
        self.play_context(spotify_id, "playlist".to_string());
    }

    fn play_context(&self, spotify_id: &String, context: String) {
        Command::new("spotify_player")
            .arg("playback")
            .arg("start")
            .arg("context")
            .arg(context)
            .arg("--id")
            .arg(spotify_id)
            .output()
            .expect("Failed to execute command");
    }

    fn set_volume(new_volume: u64) {
        Command::new("spotify_player")
            .arg("playback")
            .arg("volume")
            .arg(new_volume.to_string())
            .output()
            .expect("Failed to execute command");
    }

    fn get_current_volume() -> u64 {
        let output = Command::new("spotify_player")
            .arg("get")
            .arg("key")
            .arg("playback")
            .output()
            .expect("Failed to execute command");
        let current_playback_settings: serde_json::Value = serde_json::from_str(
            String::from_utf8(output.stdout)
                .expect("should get volume from current playback")
                .as_str(),
        )
        .expect("should have been json");
        let percent = &current_playback_settings["device"]["volume_percent"];
        percent.as_u64().unwrap()
    }
}
