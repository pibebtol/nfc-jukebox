use dotenvy::{EnvLoader, EnvSequence};

pub struct Environment {
    pub nfc_mappings_path: String,
    pub spotify_mappings_path: String,
}

impl Environment {
    pub fn new(path: &str) -> Environment {
        let env_map = EnvLoader::with_path(path)
            .sequence(EnvSequence::InputThenEnv)
            .load()
            .ok()
            .unwrap();
        let nfc_mappings_path = env_map.get("NFC_MAPPINGS_PATH").expect("should have the variable 'NFC_MAPPINGS_PATH'").to_string();
        let spotify_mappings_path = env_map.get("SPOTIFY_MAPPINGS_PATH").expect("should have the variable 'SPOTIFY_MAPPINGS_PATH'").to_string();

        Environment { nfc_mappings_path, spotify_mappings_path }
    }
}
