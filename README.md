# ComRad: Compact Radio

A cross-platform, light-weight media player.

## Features

- **Audio Playback:** Clear and seamless audio playback
- **Simplistic UI:** Clean and clutterless intuitive interface
- **Navigation:** Easy file selection
- **Single Directory Start Point:** Main directory for ease of use
- **Volume Control:** Separate audio volume control
- **Position Tracking:** Slider for position tracking and scrubbing
- **Playlist Management:** Build and manage custom playlists
- **Pop-Out Controls:** Overlay controls for ease-of-access

## Screenshots

<img width="1920" height="1020" alt="screenshot" src="https://github.com/user-attachments/assets/dba2df55-8e8e-45ef-a95f-24e8f9c87c2d" />

## Future Updates

#### Metadata

- [ ] **Edit Metadata:** Edit file metadata (title, album, artist, etc.)

#### Video

- [ ] **Video Playback:** Displays video in the central area of the application

#### Streaming

- [ ] **Stream Input:** Connect to and play streams from other sources
- [ ] **Stream Output:** Connect and stream directly to the partner mobile app (in development)

## Installation

   ```bash
   git clone https://github.com/SonusNox/comrad.git
   cd comrad
   ```

### Project Structure

```
src/
├── media/
│   ├── mod.rs
│   ├── playback.rs     # Manages playback for various media
│   └── playlist.rs     # Manages playlist object
├── utils/
│   ├── data.rs         # Transcribes media file metadata and handles stream generation
│   ├── filesys.rs      # File management (read, write, create)
│   ├── images.rs       # Image pre-loading and access
│   ├── mod.rs
│   └── styles.rs       # UI style management
├── lib.rs
└── main.rs             # Application entry, logic, and GUI management and controls
```

## Version History

- **v0.1.0:** Initial "functional" release - this is the barebones working version

    **Features**
    - Basic audio playback (including from video sources)
    - Playback within a playlist
    - Custom playlist management
    - Seamless switching between playlists and catalog
    - Directory navigation with auto-populating pseudo-playlist
    - Playback & volume controls
    - Display song title in "Now Playing" area
    - Display song title, artist, and album in "Song Info" area
    - Auto-generates config.ini file
    - Main directory and volume saved to config.ini

    **Known Issues**
    - Cannot play from all songs: A pseudo-playlist is made only when accessing a folder in the main directory
    - Pseudo-playlist changes when navigating: Changing the playlist confuses the playback code and doesn't allow the song to change until one is selected, moving playback to the new playlist

- **v0.1.1:** Shuffle feature added and playlist management updated

    **Features**
    - Shuffle button functionality added

    **Known Issues**
    - Cannot play from all songs: A pseudo-playlist is made only when accessing a folder in the main directory
    - Pseudo-playlist/playlist changes when navigating: Changing the playlist confuses the playback code and doesn't allow the song to change until one is selected, moving playback to the new playlist

- **v0.2.0:** Version update

    **Features**
    - Playback controls finished
    - Playlist controls and management finished
    - Pop-out control window added

    **Known Issues**
    - Playback controls not centered in main window

- **v0.2.1:** Minor fixes

    **Features**
    - Fixed shuffle (wasn't sorting)
    - Moved player volume setter to set volume before playback
    - Added larger pop-out player for ease of use
    - Rearranged pop-out player code
    - Rearranged player controls and "now playing" song info label

    **Known Issues**
    - None so far. Tinkering ongong.

## Dependencies

- **audiotags:** <https://github.com/tianyishi2001/audiotags>
- **eframe:** <https://github.com/emilk/egui/tree/main/crates/eframe>
- **egui:** <https://github.com/emilk/egui>
- **egui-file-dialog:** <https://github.com/jannistpl/egui-file-dialog>
- **egui_extras:** <https://github.com/emilk/egui/tree/main/crates/egui_extras>
- **image:** <https://github.com/image-rs/image/tree/main>
- **mp3-duration:** <https://github.com/agersant/mp3-duration>
- **rand:** <https://github.com/rust-random/rand>
- **rand_distr:** <https://github.com/rust-random/rand_distr>
- **rodio:** <https://github.com/RustAudio/rodio>
- **tokio:** <https://github.com/tokio-rs/tokio>
