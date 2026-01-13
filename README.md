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

<img width="1600" height="860" alt="screenshot" src="https://github.com/user-attachments/assets/20c26769-7df3-4711-890c-2865c7a7d27f" />

## Future Updates

#### Metadata

- [ ] **Edit Metadata:** Edit file metadata (title, album, artist, etc.)

#### Video

- [ ] **Video Playback:** Displays video in the central area of the application

#### Playback

- [ ] **URL Playback:** Play audio and/or video from a url source

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
├── assets/             # Contains all images used in this project
│                         (not meant to be uploaded to repo)
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