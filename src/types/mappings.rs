use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NfcMapping {
    index: u32,
    nfc_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpotifyKind {
    Album,
    Artist,
    Playlist,
    Control,
    NotSet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotifyMapping {
    nfc_index: u32,
    spotify_id: String,
    spotify_kind: SpotifyKind,
}
