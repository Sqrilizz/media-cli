# Configuration

media-cli can read defaults from:

```text
~/.config/media-cli/config.toml
```

Create the file with documented defaults:

```bash
media-cli settings --init
```

Print the default template without writing a file:

```bash
media-cli settings --defaults
```

## Example

```toml
# Default media player for video/streams. Audio deck uses mpv for IPC controls.
player = "mpv"

# Default video quality: "480p", "720p", "1080p", "best", etc.
quality = "720p"

# Play videos inline in compatible terminals by default.
terminal = false

# Default directory for local files. `~` is supported.
local_dir = "~/Videos"

# Default anime translation mode: "sub" or "dub".
anime_mode = "sub"

[music]
# Enable the in-app music deck visualizer.
visualizer = true

# Visualizer style: "mirror", "bars", or "wave".
visualizer_style = "bars"

# Visualizer sensitivity. 1.0 is neutral; try 0.7-1.8.
sensitivity = 1.15
```

`player` accepts only `mpv` or `vlc`. VLC is supported for regular local and remote video playback; terminal rendering and the Music Deck require mpv.

## Override Order

CLI arguments override config values for the command being run:

```bash
media-cli yt "video" --quality 1080p --player vlc
media-cli anime "title" --mode dub
media-cli --terminal twitch channel_name
```

If no CLI override is provided, media-cli uses `config.toml`, then built-in defaults.

## Music Visualizer Controls

Inside the Music Deck:

| Key | Action |
| --- | --- |
| `Space` | Pause/resume |
| `M` | Mute/unmute |
| `V` | Cycle visualizer style: mirror → bars → wave |
| `Q` / `Esc` | Stop playback |
