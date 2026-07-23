# 📖 Usage Examples

## YouTube

### Basic Search

```bash
media-cli yt "funny cats"
```

### Direct Link

```bash
# Full link
media-cli yt "https://youtube.com/watch?v=dQw4w9WgXcQ"

# Short link
media-cli yt "https://youtu.be/dQw4w9WgXcQ"

# With timestamp
media-cli yt "https://youtube.com/watch?v=dQw4w9WgXcQ&t=42s"
```

### Search Anime

```bash
media-cli yt "naruto opening"
```

### With Quality Selection

```bash
# 720p
media-cli yt "music video" --quality 720p

# 1080p
media-cli yt "anime" --quality 1080p

# 480p (for slow internet)
media-cli yt "podcast" --quality 480p
```

### With Different Player

```bash
media-cli yt "movie" --player vlc
```

## YouTube Music

### Audio Only Mode

```bash
# Search and play music
media-cli music "lofi hip hop"

# Play in background
media-cli music "jazz playlist"
```

## Twitch

### Watch Stream

```bash
# By channel name
media-cli twitch shroud

# By URL
media-cli twitch https://twitch.tv/shroud

# With quality
media-cli twitch xqc --quality 720p

# In terminal
media-cli twitch pokimane --terminal
```

## Anime

### Search and Watch

```bash
# Basic search
media-cli anime "naruto"

# Built-in TUI selector
media-cli anime "one piece"

# Dubbed version
media-cli anime "attack on titan" --mode dub

# With quality
media-cli anime "demon slayer" --quality 1080p
```

## Local Files

### Scan ~/Videos

```bash
media-cli file
```

### Specify Folder

```bash
media-cli file ~/Downloads
media-cli file /mnt/storage/Movies
```

### With VLC

```bash
media-cli file --player vlc
```

## History

### View History

```bash
media-cli history
```

### Clear History

```bash
media-cli history --clear
```

## Workflow Examples

### Quick Watch

```bash
# Search → select via → watch
media-cli yt "lofi hip hop"
```

### Shared Link

```bash
# Just paste the link
media-cli yt "https://youtube.com/watch?v=..."

# With quality
media-cli yt "https://youtu.be/..." --quality 1080p
```

### Local Collection

```bash
# Organize files in ~/Videos
mkdir -p ~/Videos/{Movies,Series,Anime}

# Launch
media-cli file
```

### Binge Watching Anime

```bash
# Search anime
media-cli anime "naruto"

# Select episode
# After watching, choose:
# - next: auto-play next episode
# - previous: go back
# - replay: watch again
# - select: choose different episode
```

## Configuration

```bash
# Create ~/.config/media-cli/config.toml
media-cli settings --init

# Print default config template
media-cli settings --defaults
```

Example config:

```toml
player = "mpv"
quality = "720p"
terminal = false
local_dir = "~/Videos"
anime_mode = "sub"

[music]
visualizer = true
visualizer_style = "bars"
sensitivity = 1.15
```

## Music Deck Controls

- `Space` - pause/resume
- `M` - mute/unmute
- `V` - cycle visualizer style (`mirror`, `bars`, `wave`)
- `Q` / `Esc` - stop playback

## Tips

1. supports fuzzy search - just start typing
2. Use arrow keys for navigation
3. Enter to select, Esc to cancel
4. mpv hotkeys:

   - Space - pause
   - → / ← - seek
   - f - fullscreen
   - q - quit

5. Interactive mode after playback:

   - replay - watch again
   - next - next video/episode
   - previous - previous video/episode
   - select - choose another
   - quit - exit

## Terminal Mode (kitty)

For better quality in terminal:

```bash
# Install kitty terminal
sudo apt install kitty

# Run media-cli in kitty
media-cli yt "video" -t
```

Kitty mode advantages:

- Graphical video directly in terminal
- No logs or clutter
- Improved rendering quality
- Clean output

## Advanced Usage

### Combine Options

```bash
# Anime with quality and player selection
media-cli anime "naruto" \
 --quality 1080p \
 --player mpv

# YouTube in terminal with quality
media-cli yt "music video" \
 --terminal \
 --quality 720p
```

### Create Aliases

```bash
# Add to ~/.bashrc or ~/.zshrc
alias yt='media-cli yt'
alias music='media-cli music'
alias anime='media-cli anime'
alias twitch='media-cli twitch'

# Usage
yt "funny cats"
music "lofi"
anime "naruto"
twitch "shroud"
```
