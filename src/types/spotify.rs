use serde::{Deserialize, Serialize};

pub enum SpotifyControlKind {
    Start,      // spotify_player playback start context/radio album/artist/playlist --id <id>
    PlayPause,  // spotify_player playback play-pause
    Next,       // spotify_player playback next
    Previous,   // spotify_player playback previous
    Shuffle,    // spotify_player playback shuffle
    VolumeUp,   // {spotify_player get key playback}.device.volume_percent > spotify_player playback volume (<volume>+5)
    VolumeDown, // {spotify_player get key playback}.device.volume_percent > spotify_player playback volume (<volume>-5)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpotifyKind {
    Album,
    Artist,
    Playlist,
    Control,
    NotSet,
}
