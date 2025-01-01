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
                            // only take the nfc-id
                            let nfc_id = &line[21..47];
                            self.player.play(nfc_id);
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
