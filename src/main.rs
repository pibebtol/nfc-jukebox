use std::thread;

use data_control::file_reader::FileDataProvider;
use env::environment::Environment;
use keyboard::controller::Controller;
use nfc::reader::Reader;
use spotify_player::player::PlayerControl;

mod types;
mod env;
mod data_control;
mod spotify_player;
mod keyboard;
mod nfc;

static ENV: &str = ".env";

fn main() {
    let env = Environment::new(ENV);
    let data = FileDataProvider{ env };

    let player = PlayerControl{data};

    let controller = Controller::new(player.clone());
    let keyboard_thread = thread::spawn(move || {
        controller.listen_for_key_input();
    });

    let nfc_controller = Reader { player };
    let nfc_reader_thread = thread::spawn(move || {
        nfc_controller.listen_for_nfc_tags();
    });

    keyboard_thread.join().expect("keyboard thread panicked");
    nfc_reader_thread.join().expect("nfc_reader thread panicked");
}
