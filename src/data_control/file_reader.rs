use std::fs;
use serde_json::Value;

use crate::types::mappings::{NfcMapping, SpotifyMapping};
use crate::types::data_provider::DataProvider;
use crate::env::environment::Environment;

#[derive(Clone)]
pub struct FileDataProvider {
    pub env: Environment,
}

impl DataProvider for FileDataProvider {
    fn get_nfc_mappings(&self) -> Vec<NfcMapping> {
        let result = fs::read_to_string(&self.env.nfc_mappings_path).expect("should have been able to read file");

        serde_json::from_str(&result).expect("should have been valid json")
    }

    fn get_spotify_mappings(&self) -> Vec<SpotifyMapping> {
        let result = fs::read_to_string(&self.env.spotify_mappings_path).expect("should have been able to read file");

        serde_json::from_str(&result).expect("should have been valid json")
    }
    
    fn get_spotify_albums(&self) -> Vec<Value> {
        let result = fs::read_to_string(&self.env.albums).expect("should have been able to read file");

        serde_json::from_str(&result).expect("should have been valid json")
    }
    
    fn get_spotify_artists(&self) -> Vec<Value> {
        let result = fs::read_to_string(&self.env.artists).expect("should have been able to read file");

        serde_json::from_str(&result).expect("should have been valid json")
    }
    
    fn get_spotify_playlists(&self) -> Vec<Value> {
        let result = fs::read_to_string(&self.env.playlists).expect("should have been able to read file");

        serde_json::from_str(&result).expect("should have been valid json")
    }
}
