# 🌐 Proxy & Bypass Guide

This guide explains how to use media-cli with proxies and bypass restrictions.

## Quick Start

### Auto Mode (Recommended)
```bash
# Automatically detect and use working proxy
media-cli --proxy auto yt "video"
```

### Manual Proxy
```bash
# SOCKS5
media-cli --proxy socks5://127.0.0.1:1080 yt "video"

# HTTP
media-cli --proxy http://127.0.0.1:7890 anime "title"
```

## Configuration

### proxies.txt File

Create a `proxies.txt` file in one of these locations:
- Current directory: `./proxies.txt`
- Config directory: `~/.config/media-cli/proxies.txt`
- System directory: `/usr/local/share/media-cli/proxies.txt`

Format (one proxy per line):
```
# SOCKS5 proxies
127.0.0.1:1080
socks5://proxy.example.com:1080

# HTTP proxies
http://proxy.example.com:8080

# Comments start with #
# Empty lines are ignored
```

### Auto Proxy Detection

When using `--proxy auto`, media-cli will:
1. Check if YouTube is accessible
2. If blocked, load proxies from `proxies.txt`
3. Test up to 5 random proxies
4. Use the first working proxy

```bash
# Example output
🔍 Checking YouTube access...
✕ YouTube is blocked
⚡ Searching for working proxy...
🔍 Loaded 10 proxies
  Testing socks5://127.0.0.1:1080...
✓ Working proxy: socks5://127.0.0.1:1080
```

## Common Proxy Setups

### Shadowsocks
```bash
# Default port
media-cli --proxy socks5://127.0.0.1:1080 yt "video"
```

### Clash
```bash
# HTTP port
media-cli --proxy http://127.0.0.1:7890 yt "video"

# SOCKS5 port
media-cli --proxy socks5://127.0.0.1:7891 yt "video"
```

### v2ray
```bash
# Default SOCKS5 port
media-cli --proxy socks5://127.0.0.1:10808 yt "video"

# HTTP port
media-cli --proxy http://127.0.0.1:10809 yt "video"
```

### Xray
```bash
media-cli --proxy http://127.0.0.1:10809 yt "video"
```

## DPI Bypass

### Linux (zapret)

Install zapret:
```bash
git clone https://github.com/bol-van/zapret
cd zapret
sudo ./install_easy.sh
```

zapret runs as a system service and doesn't require proxy configuration.

### Windows (GoodbyeDPI)

Download and run:
```bash
# Download from
https://github.com/ValdikSS/GoodbyeDPI

# Run as administrator
goodbyedpi.exe
```

## Usage Examples

### YouTube with Proxy
```bash
# Auto
media-cli --proxy auto yt "music video"

# Manual
media-cli --proxy socks5://127.0.0.1:1080 yt "funny cats"
```

### Anime with Proxy
```bash
media-cli --proxy auto anime "naruto"
media-cli anime "one piece" --proxy socks5://127.0.0.1:1080
```

### Music with Proxy
```bash
media-cli --proxy auto music "lofi hip hop"
```

### Twitch with Proxy
```bash
media-cli twitch shroud --proxy socks5://127.0.0.1:1080
```

## Global Proxy

Set proxy for all commands:
```bash
# Set at the beginning
media-cli --proxy socks5://127.0.0.1:1080 yt "video"
media-cli --proxy auto anime "naruto"

# Or use environment variables (for curl)
export http_proxy=http://127.0.0.1:7890
export https_proxy=http://127.0.0.1:7890
```

## Troubleshooting

### Proxy Not Working

1. Check if proxy is running:
```bash
# Test with curl
curl -x socks5://127.0.0.1:1080 https://www.youtube.com
```

2. Verify proxy address and port
3. Check firewall settings
4. Try different proxy type (SOCKS5 vs HTTP)

### Auto Mode Not Finding Proxies

1. Create `proxies.txt` file
2. Add working proxies (one per line)
3. Test proxies manually:
```bash
curl -x socks5://127.0.0.1:1080 https://www.youtube.com
```

### Slow Playback

1. Try different proxy
2. Use lower quality:
```bash
media-cli --proxy auto yt "video" --quality 720p
```
3. Check proxy bandwidth

## Security Notes

1. Only use trusted proxies
2. Avoid free public proxies (security risk)
3. Use encrypted protocols (SOCKS5, HTTPS)
4. Don't share your proxy credentials

## Advanced Configuration

### Multiple Proxies

Add multiple proxies to `proxies.txt`:
```
# Fast proxies (will be tested first)
socks5://fast-proxy.example.com:1080
socks5://backup-proxy.example.com:1080

# Slower proxies (backup)
http://slow-proxy.example.com:8080
```

Auto mode will test them randomly and use the first working one.

### Proxy Rotation

For better reliability, add multiple proxies:
```bash
# proxies.txt
socks5://proxy1.example.com:1080
socks5://proxy2.example.com:1080
socks5://proxy3.example.com:1080
```

Each time you use `--proxy auto`, a different proxy might be selected.

## Help

Show bypass help:
```bash
media-cli --bypass-help
```

Output:
```
╭─ Bypass Restrictions ──────────────────────────────╮
│                                                    │
│  1. Auto mode:                                     │
│     media-cli --proxy auto yt "query"              │
│                                                    │
│  2. DPI bypass (zapret/GoodbyeDPI):                │
│     Requires separate installation                 │
│                                                    │
│  3. Proxy/VPN:                                     │
│     media-cli --proxy socks5://127.0.0.1:1080      │
│                                                    │
│  Common proxies:                                   │
│    SOCKS5 (Shadowsocks)  socks5://127.0.0.1:1080   │
│    HTTP (Clash)          http://127.0.0.1:7890     │
│    SOCKS5 (v2ray)        socks5://127.0.0.1:10808  │
│    HTTP (Xray)           http://127.0.0.1:10809    │
│                                                    │
╰────────────────────────────────────────────────────╯
```
