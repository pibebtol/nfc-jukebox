use serde_json::Value;

use super::mappings::{NfcMapping, SpotifyMapping};


pub trait DataProvider {
    fn get_nfc_mappings(&self) -> Vec<NfcMapping>;
    fn get_spotify_mappings(&self) -> Vec<SpotifyMapping>;
    fn get_spotify_albums(&self) -> Vec<Value>;
    fn get_spotify_artists(&self) -> Vec<Value>;
    fn get_spotify_playlists(&self) -> Vec<Value>;
}
