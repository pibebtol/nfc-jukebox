use dotenvy::{EnvLoader, EnvSequence};

static NFC_MAPPINGS_PATH: &str = "NFC_MAPPINGS_PATH";
static SPOTIFY_MAPPINGS_PATH: &str = "SPOTIFY_MAPPINGS_PATH";
static SPOTIFY_ALBUMS_PATH: &str = "SPOTIFY_ALBUMS_PATH";
static SPOTIFY_ARTISTS_PATH: &str = "SPOTIFY_ARTISTS_PATH";
static SPOTIFY_PLAYLISTS_PATH: &str = "SPOTIFY_PLAYLISTS_PATH";

#[derive(Clone)]
pub struct Environment {
    pub nfc_mappings_path: String,
    pub spotify_mappings_path: String,
    pub albums: String,
    pub playlists: String,
    pub artists: String,
}

impl Environment {
    pub fn new(path: &str) -> Environment {
        let env_map = EnvLoader::with_path(path)
            .sequence(EnvSequence::InputThenEnv)
            .load()
            .ok()
            .unwrap();
        let nfc_mappings_path = env_map.get(NFC_MAPPINGS_PATH).expect("should have the variable {NFC_MAPPINGS_PATH").to_string();
        let spotify_mappings_path = env_map.get(SPOTIFY_MAPPINGS_PATH).expect("should have the variable {SPOTIFY_MAPPINGS_PATH}").to_string();
        let playlists = env_map.get(SPOTIFY_PLAYLISTS_PATH).expect("should have the env variable {SPOTIFY_PLAYLISTS_PATH}").to_string();
        let albums = env_map.get(SPOTIFY_ALBUMS_PATH).expect("should have the variable {SPOTIFY_ALBUMS_PATH}").to_string();
        let artists = env_map.get(SPOTIFY_ARTISTS_PATH).expect("should have the variable {SPOTIFY_ARTISTS_PATH}").to_string();

        Environment { nfc_mappings_path, spotify_mappings_path, albums, artists, playlists }
    }
}
