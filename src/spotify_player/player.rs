use std::process::Command;

use rand::{seq::SliceRandom, Rng};

use crate::{data_control::file_reader::FileDataProvider, types::{data_provider::DataProvider, spotify::{self, SpotifyControlKind, SpotifyKind}}};

#[derive(Clone)]
pub struct PlayerControl {
    pub data: FileDataProvider,
}

impl PlayerControl {
    pub fn play(&self, nfc_id: &str) {
        let nfc_mappings = self.data.get_nfc_mappings();
        let nfc_index = nfc_mappings.iter().find(|&mapping| mapping.nfc_id == nfc_id).unwrap().index; 

        let spotify_mappings = self.data.get_spotify_mappings();
        let spotify_mapping = spotify_mappings.iter().find(|&mapping| mapping.nfc_index == nfc_index).unwrap();

        match spotify_mapping.spotify_kind {
            SpotifyKind::Album => self.play_album(&spotify_mapping.spotify_id),
            SpotifyKind::Artist => self.play_artist(&spotify_mapping.spotify_id),
            SpotifyKind::Playlist => self.play_playlist(&spotify_mapping.spotify_id),
            _ => (),
        }

    }

    pub fn control_playback(&self, control_kind: SpotifyControlKind) {
        match control_kind {
            SpotifyControlKind::PlayPause => self.execute_control_command("play-pause".to_string()),
            SpotifyControlKind::Next => self.execute_control_command("next".to_string()),
            SpotifyControlKind::Previous => self.execute_control_command("previous".to_string()),
            SpotifyControlKind::Shuffle => self.execute_control_command("shuffle".to_string()),
            SpotifyControlKind::VolumeUp => self.volume_up(),
            SpotifyControlKind::VolumeDown => self.volume_down(),
            SpotifyControlKind::RandomAnything => self.play_random_anything(),
            SpotifyControlKind::RandomAlbum => self.play_random_album(),
            SpotifyControlKind::RandomArtist => self.play_random_artist(),
            SpotifyControlKind::RandomPlaylist => self.play_random_playlist(),
        }
    }

    fn play_random_anything(&self) {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => self.play_random_album(),
            1 => self.play_random_artist(),
            _ => self.play_random_playlist(),
        }
    }

    fn play_random_album(&self) {
        let albums = self.data.get_spotify_albums();
        let random_album = albums.choose(&mut rand::thread_rng()).unwrap();
        self.play_album(&random_album["id"].as_str().unwrap().to_string());
    }

    fn play_random_artist(&self) {
        let artists = self.data.get_spotify_artists();
        let random_artist = artists.choose(&mut rand::thread_rng()).unwrap();
        self.play_artist(&random_artist["id"].as_str().unwrap().to_string());
    }

    fn play_random_playlist(&self) {
        let playlists = self.data.get_spotify_playlists();
        let random_playlist = playlists.choose(&mut rand::thread_rng()).unwrap();
        self.play_playlist(&random_playlist["id"].as_str().unwrap().to_string());
    }

    fn execute_control_command(&self, control: String) {
        Command::new("spotify_player")
            .arg("playback")
            .arg(control)
            .output()
            .expect("Failed to execute command");
    }

    fn volume_up(&self) {
        let vol = self.get_current_volume() + 5;
        self.set_volume(vol);
    }

    fn volume_down(&self) {
        let vol = self.get_current_volume();
        if vol < 5 {
            self.set_volume(0);
        } else {
            self.set_volume(vol - 5);
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

    fn set_volume(&self, new_volume: u64) {
        Command::new("spotify_player")
            .arg("playback")
            .arg("volume")
            .arg(new_volume.to_string())
            .output()
            .expect("Failed to execute command");
    }

    fn get_current_volume(&self) -> u64 {
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
