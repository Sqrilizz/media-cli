use anyhow::{Context, Result};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Child;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct AudioMetrics {
    pub level: f64,
    pub centroid: f64,
    pub spread: f64,
    pub rolloff: f64,
}

pub struct MpvIpc {
    path: PathBuf,
}

impl MpvIpc {
    pub fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        #[cfg(unix)]
        let path =
            std::env::temp_dir().join(format!("media-cli-{}-{nonce}.sock", std::process::id()));
        #[cfg(windows)]
        let path = PathBuf::from(format!(
            r"\\.\pipe\media-cli-{}-{nonce}",
            std::process::id()
        ));
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn wait_until_ready(&self, child: &mut Child) -> Result<()> {
        for _ in 0..80 {
            if child.try_wait()?.is_some() {
                anyhow::bail!("Audio player exited before IPC became ready");
            }
            if self.get_property("pause").is_ok() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(25));
        }
        anyhow::bail!("Audio player IPC did not become ready")
    }

    pub fn set_pause(&self, paused: bool) -> Result<()> {
        self.set_property("pause", Value::Bool(paused))
    }

    pub fn set_mute(&self, muted: bool) -> Result<()> {
        self.set_property("mute", Value::Bool(muted))
    }

    pub fn metrics(&self) -> Option<AudioMetrics> {
        let data = self.get_property("af-metadata/spectrum").ok()?;
        let object = data.as_object()?;
        let rms = parse_number(object.get("lavfi.astats.Overall.RMS_level"))?;
        let peak = parse_number(object.get("lavfi.astats.Overall.Peak_level")).unwrap_or(rms);
        let rms_linear = 10_f64.powf(rms / 20.0);
        let peak_linear = 10_f64.powf(peak / 20.0);
        Some(AudioMetrics {
            level: (rms_linear * 0.72 + peak_linear * 0.28).clamp(0.0, 1.0),
            centroid: parse_number(object.get("lavfi.aspectralstats.1.centroid"))?,
            spread: parse_number(object.get("lavfi.aspectralstats.1.spread"))?,
            rolloff: parse_number(object.get("lavfi.aspectralstats.1.rolloff"))?,
        })
    }

    pub fn playback_time(&self) -> Option<u64> {
        self.get_property("time-pos")
            .ok()?
            .as_f64()
            .map(|time| time as u64)
    }

    pub fn duration(&self) -> Option<u64> {
        self.get_property("duration")
            .ok()?
            .as_f64()
            .map(|d| d as u64)
    }

    fn set_property(&self, name: &str, value: Value) -> Result<()> {
        self.request(serde_json::json!({
            "command": ["set_property", name, value]
        }))?;
        Ok(())
    }

    fn get_property(&self, name: &str) -> Result<Value> {
        self.request(serde_json::json!({
            "command": ["get_property", name]
        }))
    }

    fn request(&self, command: Value) -> Result<Value> {
        let response = request_path(&self.path, &command)?;
        if response.get("error").and_then(Value::as_str) != Some("success") {
            anyhow::bail!(
                "mpv IPC error: {}",
                response
                    .get("error")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown")
            );
        }
        Ok(response.get("data").cloned().unwrap_or(Value::Null))
    }
}

impl Drop for MpvIpc {
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = std::fs::remove_file(&self.path);
    }
}

fn parse_number(value: Option<&Value>) -> Option<f64> {
    value.and_then(|value| match value {
        Value::String(number) => number.parse().ok(),
        Value::Number(number) => number.as_f64(),
        _ => None,
    })
}

#[cfg(unix)]
fn request_path(path: &Path, command: &Value) -> Result<Value> {
    use std::os::unix::net::UnixStream;

    let mut stream = UnixStream::connect(path).context("Failed to connect to mpv IPC")?;
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    writeln!(stream, "{}", command).context("Failed to write mpv IPC command")?;
    let mut response = String::new();
    BufReader::new(stream)
        .read_line(&mut response)
        .context("Failed to read mpv IPC response")?;
    serde_json::from_str(&response).context("Invalid mpv IPC response")
}

#[cfg(windows)]
fn request_path(path: &Path, command: &Value) -> Result<Value> {
    use std::fs::OpenOptions;

    let mut stream = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .context("Failed to connect to mpv IPC")?;
    writeln!(stream, "{}", command).context("Failed to write mpv IPC command")?;
    let mut response = String::new();
    BufReader::new(stream)
        .read_line(&mut response)
        .context("Failed to read mpv IPC response")?;
    serde_json::from_str(&response).context("Invalid mpv IPC response")
}
