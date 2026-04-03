use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::sources::MediaItem;

pub fn select(items: &[MediaItem]) -> Result<Option<MediaItem>> {
    if items.is_empty() {
        return Ok(None);
    }

    let mut fzf = Command::new("fzf")
        .args([
            "--prompt=Select video: ",
            "--height=40%",
            "--reverse",
            "--border",
            "--preview=echo {}",
            "--preview-window=up:3:wrap",
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
        "quit",
        "replay",
        "next",
        "previous",
        "select",
    ];

    let mut fzf = Command::new("fzf")
        .args([
            "--prompt=Action: ",
            "--height=40%",
            "--reverse",
            "--border",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to launch fzf")?;

    {
        let stdin = fzf.stdin.as_mut().context("Failed to open stdin")?;
        for action in &actions {
            writeln!(stdin, "{}", action)?;
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

    Ok(selected)
}
