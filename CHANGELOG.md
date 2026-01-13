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

- **v0.2.2:** Minor fixes & changelog created

    **Added**
    - CHANGELOG.md file created & version history migrated

    **Changes**
    - Updated README.md file
    - Changed version update contents and layout
    - Moved code around for better functionality/readability

    **Fixes**
    - Now Playing playlist no longer remains empty when shuffle is on
    - Now Playing playlist populates from directory on initialization

    **Known Issues**
    - Now Playing playlist not properly changing when stopped if previously playing while navigating
    - Shuffle order is inconsistent during same shuffle state

- **v0.2.3:** Minor fixes

    **Added**
    - Nothing this time

    **Changes**
    - Removed code shuffling playlist every time "play" is called

    **Fixes**
    - Shuffled playlist now consistent

    **Known Issues**
    - Now Playing playlist not properly changing when stopped if previously playing while navigating