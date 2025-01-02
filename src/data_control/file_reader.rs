use serde_json::Value;
use std::fs;

use crate::env::environment::Environment;
use crate::types::data_provider::DataProvider;
use crate::types::mappings::{NfcMapping, SpotifyMapping};
use crate::types::spotify::SpotifyKind;

#[derive(Clone)]
pub struct FileDataProvider {
    pub env: Environment,
}

impl FileDataProvider {
    fn read_file(&self, path: &String) -> String {
        fs::read_to_string(path).expect("should have been able to read file")
    }
}

impl DataProvider for FileDataProvider {
    fn get_nfc_mappings(&self) -> Vec<NfcMapping> {
        serde_json::from_str(&self.read_file(&self.env.nfc_mappings_path))
            .expect("should have been valid json and match typing for NfcMapping")
    }

    fn get_spotify_mappings(&self) -> Vec<SpotifyMapping> {
        serde_json::from_str(&self.read_file(&self.env.spotify_mappings_path))
            .expect("should have been valid json and match typing for SpotifyMapping")
    }

    fn get_spotify_content(&self, kind: SpotifyKind) -> Vec<Value> {
        let content = match kind {
            SpotifyKind::Album => &self.read_file(&self.env.albums),
            SpotifyKind::Artist => &self.read_file(&self.env.artists),
            SpotifyKind::Playlist => &self.read_file(&self.env.playlists),
        };

        serde_json::from_str(&content).expect("should have been valid json")
    }
}
