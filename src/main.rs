use std::thread;

mod types;
mod env;
mod data_control;
mod spotify_player;
mod keyboard;
mod nfc;

static ENV: &str = ".env";

fn main() {
    let env = env::environment::Environment::new(ENV);
    let data = data_control::file_reader::FileDataProvider{ env };

    let player: spotify_player::player::PlayerControl = spotify_player::player::PlayerControl{data};

    let controller = keyboard::controller::Controller::new(player.clone());
    let keyboard_thread = thread::spawn(move || {
        controller.listen_for_key_input();
    });

    let nfc_controller = nfc::reader::Controller { player };
    let nfc_reader_thread = thread::spawn(move || {
        nfc_controller.listen_for_nfc_tags();
    });

    keyboard_thread.join().expect("keyboard thread panicked");
    nfc_reader_thread.join().expect("nfc_reader thread panicked");
}
