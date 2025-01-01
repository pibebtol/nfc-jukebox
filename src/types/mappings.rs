use serde::{Deserialize, Serialize};
use super::spotify::SpotifyKind;

#[derive(Serialize, Deserialize, Debug)]
pub struct NfcMapping {
    pub index: u32,
    pub nfc_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyMapping {
    pub nfc_index: u32,
    pub spotify_id: String,
    pub spotify_kind: SpotifyKind,
}
