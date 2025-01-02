use serde_json::Value;

use super::{mappings::{NfcMapping, SpotifyMapping}, spotify::SpotifyKind};


pub trait DataProvider {
    fn get_nfc_mappings(&self) -> Vec<NfcMapping>;
    fn get_spotify_mappings(&self) -> Vec<SpotifyMapping>;
    fn get_spotify_content(&self, kind: SpotifyKind) -> Vec<Value>;
}
