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

# With proxy
media-cli music "classical music" --proxy socks5://127.0.0.1:1080
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

# With GUI selector
media-cli anime "one piece" --gui

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

## Proxy & Bypass

### Auto Proxy Detection
```bash
# Automatically find working proxy
media-cli --proxy auto yt "video"

# Works with all commands
media-cli --proxy auto anime "naruto"
media-cli --proxy auto music "song"
```

### Manual Proxy
```bash
# SOCKS5
media-cli --proxy socks5://127.0.0.1:1080 yt "video"

# HTTP
media-cli --proxy http://127.0.0.1:7890 anime "title"
```

### Bypass Help
```bash
# Show bypass options
media-cli --bypass-help
```

## Workflow Examples

### Quick Watch
```bash
# Search → select via fzf → watch
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
media-cli anime "naruto" --gui

# Select episode
# After watching, choose:
# - next: auto-play next episode
# - previous: go back
# - replay: watch again
# - select: choose different episode
```

## Tips

1. fzf supports fuzzy search - just start typing
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
# Anime with all options
media-cli anime "naruto" \
  --quality 1080p \
  --player mpv \
  --proxy socks5://127.0.0.1:1080 \
  --gui

# YouTube in terminal with quality
media-cli yt "music video" \
  --terminal \
  --quality 720p \
  --proxy auto
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
