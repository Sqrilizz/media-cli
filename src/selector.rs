use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::sources::MediaItem;

const FZF_COLORS: &str = "--color=bg+:#2d2b55,fg+:#ffffff,hl:#ff79c6,hl+:#ff79c6,pointer:#bd93f9,prompt:#82b4ff,info:#6272a4,border:#44475a,header:#6272a4";

pub fn select(items: &[MediaItem]) -> Result<Option<MediaItem>> {
    if items.is_empty() {
        return Ok(None);
    }

    let mut fzf = Command::new("fzf")
        .args([
            "--prompt=  ▸ Select: ",
            "--height=50%",
            "--reverse",
            "--border=rounded",
            "--margin=1,2",
            "--padding=1",
            "--pointer=▸",
            "--marker=●",
            FZF_COLORS,
            "--header=  ↑↓ navigate │ Enter select │ Esc cancel",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to launch fzf. Install: sudo apt install fzf")?;

    {
        let stdin = fzf.stdin.as_mut().context("Failed to open stdin")?;
        for (idx, item) in items.iter().enumerate() {
            writeln!(stdin, "[{}] {}", idx, item)?;
        }
    }

    let output = fzf.wait_with_output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let selected_line = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected_line.is_empty() {
        return Ok(None);
    }

    if let Some(idx_str) = selected_line.split(']').next().and_then(|s| s.strip_prefix('[')) {
        if let Ok(idx) = idx_str.parse::<usize>() {
            if idx < items.len() {
                return Ok(Some(items[idx].clone()));
            }
        }
    }

    Ok(None)
}

pub fn select_action() -> Result<String> {
    let actions = vec![
        ("replay",   "↻  Replay        ─ watch again"),
        ("next",     "▸  Next          ─ play next item"),
        ("previous", "◂  Previous      ─ play previous item"),
        ("select",   "≡  Select        ─ choose another"),
        ("quit",     "✕  Quit          ─ back to menu"),
    ];

    let mut fzf = Command::new("fzf")
        .args([
            "--prompt=  ▸ Action: ",
            "--height=30%",
            "--reverse",
            "--border=rounded",
            "--margin=1,2",
            "--padding=1",
            "--pointer=▸",
            "--no-info",
            FZF_COLORS,
            "--header=  What would you like to do?",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to launch fzf")?;

    {
        let stdin = fzf.stdin.as_mut().context("Failed to open stdin")?;
        for (key, label) in &actions {
            writeln!(stdin, "[{}] {}", key, label)?;
        }
    }

    let output = fzf.wait_with_output()?;

    if !output.status.success() {
        return Ok("quit".to_string());
    }

    let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if selected.is_empty() {
        return Ok("quit".to_string());
    }

    if let Some(key) = selected.split(']').next().and_then(|s| s.strip_prefix('[')) {
        return Ok(key.to_string());
    }

    Ok("quit".to_string())
}
