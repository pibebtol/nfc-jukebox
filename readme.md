# nfc-jukebox

## ToDo
### base
- [x] read mappings
- [x] execute command line stuff
- [x] get nfc-reader input
- [x] get keyboard input
- [x] threading

### proper
- [ ] logging
- [x] get rid of dotenvy? - no, it's actually nice, to add environment variables from console
- [x] get rid of crossterm?
- [x] control spotify-player
    - [x] volume
    - [x] play tags
    - [x] control
    - [x] play random stuff
- [x] nfc control
- [x] keyboard input
- [ ] figure out deployment
  - [x] setup tmux
  - [ ] figure out how to get keyboard input
- [ ] general control
  - [x] restart spotify player
  - [ ] check network?
  - [ ] shut down?

## spotify_player

Used here to play the music.

Adjust default configuration via `~/.config/spotify-player/app.toml`, e.g. change
default name, volume or autoplay.

### Get spotify data:
```
spotify_player get key user-playlists > data/spotify_playlists.json
spotify_player get key user-saved-albums > spotify_albums.json
spotify_player get key user-followed-artists > spotify_artists.json
```

## Setup

1. create the `nfc_mappings.json` and `spotify_mappings.json` file following the example template files
2. fill the `nfc_mappings.json` with your nfc-tag-ids
  a. if ACR122u is used, following or similar commands can be used to get the nfc-ids
  ```
  sudo modprobe -r pn533_usb
  while true; nfc-poll | grep NFCID1 | sed 's/\(.*:\s\)\(.*\)\s\s/\2/' | xclip -selection clipboard; end
  ```
3. fill the `spotify_mappings.json` with your desired album/artist/playlist and assign to the nfc-index
4. put the app on the respective device.


copy autostart to `/etc`:`cp juke-the-box.desktop /etc/xdg/autostart/juke-the-box.desktop` 
