use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::spotify_player::player::PlayerControl;

pub struct Reader {
    pub player: PlayerControl,
}

impl Reader {
    pub fn listen_for_nfc_tags(&self) {
        loop {
            // wraps nfc-poll into stdbuf, to get the buffer output instantly
            // (else it waits for process finish, which only happens on nfc-tag-removal)
            let mut child = Command::new("stdbuf")
                .arg("-oL")
                .arg("nfc-poll")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let stdout = child.stdout.take().expect("Failed to open stdout");

            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line.contains("NFCID1") {
                            // only take the nfc-id from the line
                            self.player.play(&line[21..47]);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading line: {}", e);
                    }
                }
            }

            // wait for nfc-poll to finish
            let _ = child.wait().unwrap();
        }
    }
}
