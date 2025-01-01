use crate::{NfcMapping, SpotifyMapping};

pub trait DataProvider {
    fn get_nfc_mappings(&self) -> Vec<NfcMapping>;
    fn get_spotify_mappings(&self) -> Vec<SpotifyMapping>;
}
