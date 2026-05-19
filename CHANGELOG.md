# Changelog

## [Unreleased]

### Fixed
- **Anime episode 1 now available**: Fixed episode parsing that was filtering out episode 1
  - Issue: sed command wasn't removing square brackets properly
  - Solution: Updated regex to `s|[][]||g` to remove all brackets
  - Now all episodes from 1 onwards are correctly parsed

### Changed
- Simplified episode filtering logic in Rust code
- Improved sed command for better episode extraction

## [0.1.0] - 2024-04-03

### Added
- Initial release
- YouTube video playback
- YouTube Music (audio-only mode)
- Anime support via allanime API
- Twitch stream support
- Local file playback
- History tracking
- Proxy/VPN support with auto-detection
- Interactive menu with fzf
- Multiple platform binaries (Linux, macOS, Windows)
- Automatic GitHub Actions releases
- Pre-compiled binary in repository

### Features
- Search and play YouTube videos
- Audio-only music mode
- Watch anime with multiple providers
- Watch Twitch live streams
- Play local video files
- Track watch history
- Bypass restrictions with proxy
- Interactive selection with fzf
- Terminal playback mode (kitty)
- Quality selection
- Player selection (mpv, vlc)
