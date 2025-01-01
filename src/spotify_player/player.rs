use std::process::Command;

pub struct Player {}

impl Player {
    pub fn new() -> Player {
        Player {}
    }

    pub fn play(&self) {
        let output = Command::new("spotify_player")
            .arg("playback")
            .arg("start")
            .arg("context")
            .arg("album")
            .arg("--id")
            .arg("3jPF3K9Uar57XfdHZiToKa")
            .output()
            .expect("Failed to execute command");
    }
}
