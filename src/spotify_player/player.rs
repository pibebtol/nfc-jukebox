use std::process::Command;

enum SpotifyControlKind {
    Start, // spotify_player playback start context/radio album/artist/playlist --id <id>
    PlayPause, // spotify_player playback play-pause
    Next, // spotify_player playback next
    Previous, // spotify_player playback previous
    Shuffle, // spotify_player playback shuffle
    Volume, // {spotify_player get key playback}.device.volume_percent > spotify_player playback volume (<volume>+/-5)
}

pub struct PlayerControl {}

impl PlayerControl {
    pub fn new() -> PlayerControl {
        PlayerControl {}
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
    
    pub fn volume_up(&self) {
        let vol = Self::get_current_volume() + 5;
        Self::set_volume(vol);
    }

    pub fn volume_down(&self) {
        let vol = Self::get_current_volume() - 5;
        Self::set_volume(vol);
    }

    fn set_volume(new_volume: u64) {
        let output = Command::new("spotify_player")
            .arg("playback")
            .arg("volume")
            .arg(new_volume.to_string())
            .output()
            .expect("Failed to execute command");
    }

    fn get_current_volume() -> u64 {
        let output = Command::new("spotify_player")
            .arg("get")
            .arg("key")
            .arg("playback")
            .output()
            .expect("Failed to execute command");
        let current_playback_settings: serde_json::Value = serde_json::from_str(String::from_utf8(output.stdout).expect("should get volume from current playback").as_str()).expect("should have been json");
        let percent = &current_playback_settings["device"]["volume_percent"];
        percent.as_u64().unwrap()
    }

}
