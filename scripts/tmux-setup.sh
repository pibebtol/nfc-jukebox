#!/bin/bash

echo "[juke-the-box.sh] start new tmux session with the jukebox and spotify"

# Start a new tmux session
tmux new-session -d -s nfc-jukebox

# Create the first window and run spotify_player
tmux send-keys -t nfc-jukebox:1 '~/workspace/nfc-jukebox/spotify-starter.sh' C-m

# Create a second window and run the nfc-jukebox script
tmux new-window -t nfc-jukebox:2 -n
tmux send-keys -t nfc-jukebox:2 '~/workspace/nfc-jukebox/target/release/nfc-jukebox' C-m

echo "[juke-the-box.sh] created new tmux session"
