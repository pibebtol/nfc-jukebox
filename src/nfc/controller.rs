use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};

pub struct Controller {}

impl Controller {
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

            let reader_thread = thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            println!("Output: {}", line);
                        }
                        Err(e) => {
                            eprintln!("Error reading line: {}", e);
                        }
                    }
                }
            });

            reader_thread.join().expect("Reader thread panicked");

            // wait for nfc-poll to finish
            let _ = child.wait().unwrap();
        }
    }
}
