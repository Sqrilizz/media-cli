use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::io;
use std::io::Read;
use std::process::Child;
use std::time::Duration;

use crate::config::{MusicConfig, VisualizerStyle};

struct TerminalCleanup {
    active: bool,
}

impl TerminalCleanup {
    fn enter() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        if let Err(error) = execute!(stdout, EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(error.into());
        }
        Ok(Self { active: true })
    }
}

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        if self.active {
            let mut stdout = io::stdout();
            let _ = execute!(stdout, LeaveAlternateScreen);
            let _ = disable_raw_mode();
        }
    }
}
use crate::sources::MediaItem;

#[derive(Clone, Copy)]
pub enum HomeAction {
    YouTube,
    Music,
    Twitch,
    Anime,
    Local,
    History,
    Settings,
    Quit,
}

const ROSE: Color = Color::Rgb(255, 255, 255);
const PINK: Color = Color::Rgb(184, 154, 255);
const MAUVE: Color = Color::Rgb(126, 157, 255);
const BLUE: Color = Color::Rgb(74, 169, 255);
const SAPPHIRE: Color = Color::Rgb(71, 213, 255);
const TEAL: Color = Color::Rgb(91, 232, 214);
const GREEN: Color = Color::Rgb(89, 224, 168);
const YELLOW: Color = Color::Rgb(255, 202, 92);
const _PEACH: Color = Color::Rgb(255, 153, 92);
const RED: Color = Color::Rgb(255, 102, 128);
const TEXT: Color = Color::Rgb(230, 238, 255);
const _SUBTEXT: Color = Color::Rgb(166, 186, 218);
const MUTED: Color = Color::Rgb(122, 143, 177);
const SURFACE: Color = Color::Rgb(14, 22, 38);
const SURFACE1: Color = Color::Rgb(24, 38, 63);
const OVERLAY: Color = Color::Rgb(48, 72, 109);
const BG: Color = Color::Rgb(5, 10, 20);

const HOME_ACTIONS: &[(HomeAction, &str, &str, &str)] = &[
    (
        HomeAction::YouTube,
        "Video",
        "Search YouTube or open a link",
        "▶",
    ),
    (HomeAction::Music, "Music", "Focused audio playback", "♪"),
    (HomeAction::Twitch, "Live", "Open a Twitch channel", "●"),
    (HomeAction::Anime, "Anime", "Browse shows and episodes", "◆"),
    (HomeAction::Local, "Files", "Browse your media library", "■"),
    (
        HomeAction::History,
        "Recents",
        "Continue where you left off",
        "↶",
    ),
    (
        HomeAction::Settings,
        "Settings",
        "Playback preferences and defaults",
        "⚙",
    ),
    (HomeAction::Quit, "Quit", "Close media-cli", "×"),
];

pub fn run_home(version: &str, terminal_mode: bool) -> Result<HomeAction> {
    let _cleanup = TerminalCleanup::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_home_app(&mut terminal, version, terminal_mode);
    terminal.show_cursor()?;
    result
}

fn run_home_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    version: &str,
    terminal_mode: bool,
) -> Result<HomeAction> {
    let mut state = ListState::default();
    state.select(Some(0));
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let compact = area.width < 82 || area.height < 22;
            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(if compact { 5 } else { 6 }),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .margin(if compact { 0 } else { 1 })
                .split(area);

            let mode = if terminal_mode { "INLINE" } else { "WINDOW" };
            let header = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled(
                        "  MEDIA",
                        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        ".CLI",
                        Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  YOUR MEDIA, IN FOCUS", Style::default().fg(MUTED)),
                ]),
                Line::from(Span::styled(
                    "One place for video, music, live streams and local files",
                    Style::default().fg(MUTED),
                )),
                Line::from(vec![
                    Span::styled(
                        format!("  v{}  ", version),
                        Style::default().fg(TEXT).bg(SURFACE1),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!("  {}  ", mode),
                        Style::default().fg(BLUE).bg(SURFACE1),
                    ),
                ]),
            ])
            .block(
                Block::default()
                    .title(Span::styled(" MEDIA CLI ", Style::default().fg(BLUE)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(BLUE))
                    .style(Style::default().bg(SURFACE)),
            );
            frame.render_widget(header, sections[0]);

            let body = if compact {
                vec![sections[1]]
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
                    .split(sections[1])
                    .to_vec()
            };
            let items = HOME_ACTIONS
                .iter()
                .enumerate()
                .map(|(index, (_, title, description, icon))| {
                    let number = format!(" {:02} ", index + 1);
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            number,
                            Style::default().fg(OVERLAY).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{}  ", icon),
                            Style::default().fg(if index % 2 == 0 { ROSE } else { BLUE }),
                        ),
                        Span::styled(
                            format!("{:<13}", title),
                            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            if compact { *description } else { "" },
                            Style::default().fg(MUTED),
                        ),
                    ]))
                })
                .collect::<Vec<_>>();
            let menu = List::new(items)
                .block(
                    Block::default()
                        .title(Span::styled(" LIBRARY ", Style::default().fg(MAUVE)))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(OVERLAY)),
                )
                .highlight_symbol("┃")
                .highlight_style(
                    Style::default()
                        .fg(TEXT)
                        .bg(SURFACE1)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_stateful_widget(menu, body[0], &mut state);

            if !compact {
                let selected = state.selected().unwrap_or(0);
                let (_, title, description, icon) = HOME_ACTIONS[selected];
                let detail = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(
                            format!("  {}  ", icon),
                            Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            title.to_uppercase(),
                            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled(
                        format!("  {description}"),
                        Style::default().fg(MUTED),
                    )),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  STATUS  ", Style::default().fg(GREEN).bg(SURFACE1)),
                        Span::styled(
                            "  READY",
                            Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
                        ),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled(
                        "  Press Enter to launch",
                        Style::default().fg(BLUE),
                    )),
                ])
                .block(
                    Block::default()
                        .title(Span::styled(" OVERVIEW ", Style::default().fg(BLUE)))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(SURFACE1))
                        .style(Style::default().bg(SURFACE)),
                );
                frame.render_widget(detail, body[1]);
            }

            let footer = Paragraph::new(Line::from(vec![
                Span::styled(
                    " ↑↓ / JK ",
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                ),
                Span::styled("navigate   ", Style::default().fg(MUTED)),
                Span::styled(
                    " Enter ",
                    Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                ),
                Span::styled("launch   ", Style::default().fg(MUTED)),
                Span::styled(
                    " q ",
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                ),
                Span::styled("quit", Style::default().fg(MUTED)),
            ]))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(OVERLAY)),
            );
            frame.render_widget(footer, sections[2]);
        })?;

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            let selected = state.selected().unwrap_or(0);
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                return Ok(HomeAction::Quit);
            }
            match code {
                KeyCode::Down | KeyCode::Char('j') => {
                    state.select(Some((selected + 1) % HOME_ACTIONS.len()))
                }
                KeyCode::Up | KeyCode::Char('k') => state.select(Some(if selected == 0 {
                    HOME_ACTIONS.len() - 1
                } else {
                    selected - 1
                })),
                KeyCode::Enter => return Ok(HOME_ACTIONS[selected].0),
                KeyCode::Char('1') => return Ok(HomeAction::YouTube),
                KeyCode::Char('2') => return Ok(HomeAction::Music),
                KeyCode::Char('3') => return Ok(HomeAction::Twitch),
                KeyCode::Char('4') => return Ok(HomeAction::Anime),
                KeyCode::Char('5') => return Ok(HomeAction::Local),
                KeyCode::Char('6') => return Ok(HomeAction::History),
                KeyCode::Char('7') => return Ok(HomeAction::Settings),
                KeyCode::Esc | KeyCode::Char('q') => return Ok(HomeAction::Quit),
                _ => {}
            }
        }
    }
}

pub fn run_music_player(
    title: &str,
    child: &mut Child,
    ipc: &crate::mpv_ipc::MpvIpc,
    music_config: &MusicConfig,
) -> Result<()> {
    let _cleanup = TerminalCleanup::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_music_player_app(&mut terminal, title, child, ipc, music_config);
    terminal.show_cursor()?;
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result
}

fn run_music_player_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    title: &str,
    child: &mut Child,
    ipc: &crate::mpv_ipc::MpvIpc,
    music_config: &MusicConfig,
) -> Result<()> {
    let mut paused = false;
    let mut muted = false;
    let mut visualizer = VisualizerState::new(music_config);

    loop {
        if let Some(status) = child.try_wait()? {
            if status.success() {
                return Ok(());
            }
            let mut stderr_buf = String::new();
            if let Some(mut stderr) = child.stderr.take() {
                let _ = stderr.read_to_string(&mut stderr_buf);
            }
            if stderr_buf.trim().is_empty() {
                anyhow::bail!("Audio player exited with error (status: {})", status);
            }
            anyhow::bail!("Audio player exited with error: {}", stderr_buf.trim());
        }

        if !paused {
            if let Some(metrics) = ipc.metrics() {
                visualizer.update(&metrics);
            } else {
                visualizer.decay();
            }
        } else {
            visualizer.decay();
        }
        let elapsed = ipc.playback_time().unwrap_or_default();
        let total = ipc.duration().unwrap_or(0);

        terminal.draw(|frame| {
            let area = frame.size();
            let w = area.width as usize;
            let h = area.height as usize;
            let compact = w < 60 || h < 16;

            // ── Layout ─────────────────────────────────────────────
            //  Row 0: header         (2 lines)
            //  Row 1: track info     (2 lines)
            //  Row 2: visualizer     (flex)
            //  Row 3: progress bar   (1 line)
            //  Row 4: status bar     (1 line)
            let sections = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2), // header
                    Constraint::Length(2), // track
                    Constraint::Min(4),    // visualizer
                    Constraint::Length(1), // progress
                    Constraint::Length(1), // status
                ])
                .split(area);

            // ── Header ────────────────────────────────────────────
            let header_lines = if compact {
                vec![Line::from(vec![
                    Span::styled(
                        " MEDIA CLI ",
                        Style::default().fg(MAUVE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        if paused { "▌ PAUSED" } else { "▶ PLAYING" },
                        Style::default().fg(if paused { YELLOW } else { GREEN }),
                    ),
                ])]
            } else {
                vec![Line::from(vec![
                    Span::styled(
                        " MEDIA CLI ",
                        Style::default().fg(MAUVE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("// MUSIC DECK", Style::default().fg(MUTED)),
                    Span::styled(
                        if paused {
                            "  ▌ PAUSED"
                        } else {
                            "  ▶ PLAYING"
                        },
                        Style::default().fg(if paused { YELLOW } else { GREEN }),
                    ),
                ])]
            };
            let header = Paragraph::new(header_lines).style(Style::default().bg(SURFACE));
            frame.render_widget(header, sections[0]);

            // ── Track info ────────────────────────────────────────
            let track_spans = if compact {
                vec![Line::from(Span::styled(
                    title,
                    Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                ))]
            } else {
                vec![
                    Line::from(Span::styled(
                        title,
                        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                    )),
                    Line::from(vec![Span::styled(
                        if muted { "🔇 MUTED" } else { "" },
                        Style::default().fg(RED),
                    )]),
                ]
            };
            let track = Paragraph::new(track_spans)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .style(Style::default().bg(SURFACE));
            frame.render_widget(track, sections[1]);

            // ── Visualizer ────────────────────────────────────────
            render_visualizer(frame, sections[2], &visualizer);

            // ── Progress bar ──────────────────────────────────────
            let progress_area = sections[3];
            let bar_w = progress_area.width as usize;
            if bar_w >= 4 && total > 0 {
                let pct = (elapsed as f64 / total as f64).clamp(0.0, 1.0);

                let time_left = format!("{:02}:{:02}", elapsed / 60, elapsed % 60);
                let time_right = format!("{:02}:{:02}", total / 60, total % 60);
                let time_gap = 2 + time_left.len() + time_right.len(); // spaces + times
                let bar_inner = bar_w.saturating_sub(time_gap);
                let bar_fill = ((pct * bar_inner as f64).round() as usize).min(bar_inner);

                let mut spans = Vec::with_capacity(bar_w);
                spans.push(Span::styled(
                    format!(" {} ", time_left),
                    Style::default().fg(MUTED),
                ));
                for i in 0..bar_inner {
                    let (glyph, color) = if i < bar_fill {
                        ("━", if pct > 0.85 { RED } else { BLUE })
                    } else {
                        ("─", OVERLAY)
                    };
                    spans.push(Span::styled(glyph, Style::default().fg(color)));
                }
                spans.push(Span::styled(
                    format!(" {} ", time_right),
                    Style::default().fg(MUTED),
                ));
                frame.render_widget(Paragraph::new(Line::from(spans)), progress_area);
            } else if bar_w >= 4 {
                // No duration info yet — show elapsed only
                let time_str = format!(" {:02}:{:02} ", elapsed / 60, elapsed % 60);
                let pad = bar_w.saturating_sub(time_str.len());
                let display = format!("{}{:<pad$}", time_str, "");
                frame.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        &display[..bar_w.min(display.len())],
                        Style::default().fg(MUTED),
                    ))),
                    progress_area,
                );
            }

            // ── Status bar ────────────────────────────────────────
            let status = Paragraph::new(Line::from(vec![
                Span::styled(
                    " SPACE ",
                    Style::default().fg(MAUVE).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if paused { "play " } else { "pause " },
                    Style::default().fg(MUTED),
                ),
                Span::styled(" M ", Style::default().fg(RED).add_modifier(Modifier::BOLD)),
                Span::styled(
                    if muted { "unmute " } else { "mute " },
                    Style::default().fg(MUTED),
                ),
                Span::styled(
                    " V ",
                    Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                ),
                Span::styled("vis ", Style::default().fg(MUTED)),
                Span::styled(" Q ", Style::default().fg(RED).add_modifier(Modifier::BOLD)),
                Span::styled("quit", Style::default().fg(MUTED)),
            ]))
            .style(Style::default().bg(SURFACE));
            frame.render_widget(status, sections[4]);
        })?;

        if event::poll(Duration::from_millis(33))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Ok(());
                }
                match code {
                    KeyCode::Char(' ') => {
                        let next = !paused;
                        ipc.set_pause(next)?;
                        paused = next;
                    }
                    KeyCode::Char('m') | KeyCode::Char('M') => {
                        let next = !muted;
                        ipc.set_mute(next)?;
                        muted = next;
                    }
                    KeyCode::Char('v') | KeyCode::Char('V') => {
                        visualizer.toggle_style();
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        let _ = child.kill();
                        let _ = child.wait();
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

struct VisualizerState {
    enabled: bool,
    style: VisualizerStyle,
    sensitivity: f64,
    signal: Vec<f64>,
    peaks: Vec<f64>,
    frame: u64,
}

impl VisualizerState {
    fn new(config: &MusicConfig) -> Self {
        Self {
            enabled: config.visualizer,
            style: config.visualizer_style,
            sensitivity: config.sensitivity.clamp(0.2, 3.0),
            signal: vec![0.0; 192],
            peaks: vec![0.0; 192],
            frame: 0,
        }
    }

    fn update(&mut self, metrics: &crate::mpv_ipc::AudioMetrics) {
        self.frame = self.frame.wrapping_add(1);
        if !self.enabled {
            return;
        }

        let amplitude = (metrics.level.sqrt() * 28.0 * self.sensitivity).clamp(0.0, 32.0);
        let width = metrics.spread.max(90.0);
        let denominator = self.signal.len().saturating_sub(1).max(1) as f64;
        let phase = self.frame as f64 * 0.075;
        for (index, bar) in self.signal.iter_mut().enumerate() {
            let position = index as f64 / denominator;
            let frequency = 35.0 * 460_f64.powf(position);
            let center = gaussian(frequency, metrics.centroid, width);
            let shoulder = gaussian(frequency, metrics.rolloff, width * 1.65);
            let bass = gaussian(frequency, 95.0, 170.0) * 0.16;
            let wide_wave =
                ((position * std::f64::consts::TAU * 2.0 + phase).sin() * 0.5 + 0.5) * 0.16;
            let counter_wave =
                ((position * std::f64::consts::TAU * 6.0 - phase * 1.4).sin() * 0.5 + 0.5) * 0.09;
            let edge_lift = 0.16;
            let spectral = center.max(shoulder * 0.50).max(bass);
            let target =
                (amplitude * (spectral + wide_wave + counter_wave + edge_lift)).clamp(0.0, 32.0);
            let attack = if target > *bar { 0.55 } else { 0.16 };
            *bar = *bar * (1.0 - attack) + target * attack;
            if let Some(peak) = self.peaks.get_mut(index) {
                *peak = peak.max(*bar) * 0.982;
            }
        }
    }

    fn decay(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        for bar in &mut self.signal {
            *bar *= 0.92;
        }
        for peak in &mut self.peaks {
            *peak *= 0.965;
        }
    }

    fn toggle_style(&mut self) {
        self.style = match self.style {
            VisualizerStyle::Mirror => VisualizerStyle::Bars,
            VisualizerStyle::Bars => VisualizerStyle::Wave,
            VisualizerStyle::Wave => VisualizerStyle::Mirror,
        };
    }

    fn style_label(&self) -> &'static str {
        match self.style {
            VisualizerStyle::Mirror => "MIRROR",
            VisualizerStyle::Bars => "BARS",
            VisualizerStyle::Wave => "WAVE",
        }
    }
}

fn gaussian(value: f64, center: f64, width: f64) -> f64 {
    (-0.5 * ((value - center) / width).powi(2)).exp()
}

#[cfg(test)]
mod visualizer_tests {
    use super::*;

    #[test]
    fn visualizer_cycles_every_style() {
        let config = MusicConfig::default();
        let mut state = VisualizerState::new(&config);
        state.style = VisualizerStyle::Mirror;
        state.toggle_style();
        assert_eq!(state.style, VisualizerStyle::Bars);
        state.toggle_style();
        assert_eq!(state.style, VisualizerStyle::Wave);
        state.toggle_style();
        assert_eq!(state.style, VisualizerStyle::Mirror);
    }
}

fn render_visualizer(frame: &mut ratatui::Frame, area: Rect, visualizer: &VisualizerState) {
    let title = format!(
        " VISUALIZER / {} / {:.1}x ",
        visualizer.style_label(),
        visualizer.sensitivity
    );
    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(MAUVE)))
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(OVERLAY))
        .style(Style::default().bg(SURFACE));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if !visualizer.enabled {
        let disabled = Paragraph::new("Visualizer disabled in config.toml")
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED));
        frame.render_widget(disabled, inner);
        return;
    }

    match visualizer.style {
        VisualizerStyle::Mirror => render_mirror_visualizer(frame, inner, visualizer),
        VisualizerStyle::Bars => render_bar_visualizer(frame, inner, visualizer),
        VisualizerStyle::Wave => render_wave_visualizer(frame, inner, visualizer),
    }
}

fn render_wave_visualizer(frame: &mut ratatui::Frame, area: Rect, visualizer: &VisualizerState) {
    if area.width == 0 || area.height == 0 || visualizer.signal.is_empty() {
        return;
    }
    let rows = area.height as usize;
    let center = (rows.saturating_sub(1) as f64) / 2.0;
    let amplitude = center.max(1.0);
    let mut lines = Vec::with_capacity(rows);
    for row in 0..rows {
        let mut line = String::with_capacity(area.width as usize);
        for column in 0..area.width as usize {
            let level = interpolate_signal(&visualizer.signal, column, area.width as usize) / 32.0;
            let target = center - (level - 0.5) * amplitude * 1.8;
            line.push(if (row as f64 - target).abs() < 0.6 {
                '●'
            } else {
                ' '
            });
        }
        lines.push(Line::styled(line, Style::default().fg(MAUVE)));
    }
    frame.render_widget(Paragraph::new(lines), area);
}

fn render_mirror_visualizer(frame: &mut ratatui::Frame, area: Rect, visualizer: &VisualizerState) {
    if area.width == 0 || area.height == 0 || visualizer.signal.is_empty() {
        return;
    }

    // CAVA-style mirror: solid blocks growing symmetrically from center.
    let bar_width: usize = 2;
    let bar_spacing: usize = 1;
    let total_per_bar = bar_width + bar_spacing;
    let terminal_width = area.width as usize;
    let rows = area.height as usize;
    let num_bars = (terminal_width / total_per_bar).max(1);

    let total_levels = rows * 8;
    let center_level = total_levels / 2;

    // Bottom-fill fragments (bar grows upward into this cell).
    const FRAG_BOT: [&str; 8] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇"];
    // Top-fill fragments (bar grows downward into this cell).
    const FRAG_TOP: [&str; 8] = [
        " ",
        "\u{2594}", // ▔ 1/8 from top
        "\u{1FB82}",
        "\u{1FB83}",
        "\u{2580}", // ▀ 4/8 from top
        "\u{1FB84}",
        "\u{1FB85}",
        "\u{1FB86}",
    ];

    let lines: Vec<Line> = (0..rows)
        .map(|row| {
            let mut spans = Vec::with_capacity(terminal_width);
            let row_from_bottom = rows - 1 - row;

            for bar_idx in 0..num_bars {
                let level = interpolate_signal(&visualizer.signal, bar_idx, num_bars) / 32.0;
                let peak = interpolate_signal(&visualizer.peaks, bar_idx, num_bars) / 32.0;

                let extent = (level.clamp(0.0, 1.0) * center_level as f64).round() as usize;
                let peak_ext = (peak.clamp(0.0, 1.0) * center_level as f64).ceil() as usize;

                let bar_top = (center_level + extent).min(total_levels);
                let bar_bottom = center_level.saturating_sub(extent);

                let cell_bottom = row_from_bottom * 8;
                let cell_top = cell_bottom + 8;
                let in_top_half = (cell_bottom + cell_top) / 2 >= center_level;

                for _ in 0..bar_width {
                    // Overlap between bar range and this cell.
                    let ov_start = bar_bottom.max(cell_bottom);
                    let ov_end = bar_top.min(cell_top);
                    let fill = ov_end.saturating_sub(ov_start);

                    let (glyph, color) = if fill >= 8 {
                        ("█", mirror_color(row as u16, area.height))
                    } else if fill > 0 {
                        let frag = if in_top_half {
                            FRAG_BOT[fill]
                        } else {
                            FRAG_TOP[fill]
                        };
                        (frag, mirror_color(row as u16, area.height))
                    } else if peak > 0.02 && peak_ext > extent + 8 {
                        // Peak cap on the outer edge.
                        let pk_top = (center_level + peak_ext).min(total_levels);
                        let pk_bot = center_level.saturating_sub(peak_ext);
                        let pk_ov = pk_bot.max(cell_bottom) < pk_top.min(cell_top);
                        if pk_ov {
                            ("▔", RED)
                        } else {
                            (" ", BG)
                        }
                    } else {
                        (" ", BG)
                    };

                    spans.push(Span::styled(glyph, Style::default().fg(color)));
                }

                for _ in 0..bar_spacing {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            let used = num_bars * total_per_bar;
            if used < terminal_width {
                for _ in 0..(terminal_width - used) {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            Line::from(spans)
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_bar_visualizer(frame: &mut ratatui::Frame, area: Rect, visualizer: &VisualizerState) {
    if area.width == 0 || area.height == 0 || visualizer.signal.is_empty() {
        return;
    }

    // CAVA-style: bars grow from bottom, sub-cell resolution,
    // configurable bar width + spacing for breathing room.
    let bar_width: usize = 2;
    let bar_spacing: usize = 1;
    let total_per_bar = bar_width + bar_spacing;
    let terminal_width = area.width as usize;
    let rows = area.height as usize;
    let num_bars = (terminal_width / total_per_bar).max(1);

    // 8 sub-levels per terminal row for smooth partial fills.
    // Bottom fragments: fill from bottom of cell.
    const FRAG: [&str; 8] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇"];

    let lines: Vec<Line> = (0..rows)
        .map(|row| {
            let mut spans = Vec::with_capacity(terminal_width);
            let row_from_bottom = rows - 1 - row;

            for bar_idx in 0..num_bars {
                let level = interpolate_signal(&visualizer.signal, bar_idx, num_bars) / 32.0;
                let peak = interpolate_signal(&visualizer.peaks, bar_idx, num_bars) / 32.0;

                let filled = (level.clamp(0.0, 1.0) * (rows as f64 * 8.0)).round() as usize;
                let peak_level = (peak.clamp(0.0, 1.0) * (rows as f64 * 8.0)).ceil() as usize;

                let cell_bottom = row_from_bottom * 8;
                let cell_top = cell_bottom + 8;

                for _ in 0..bar_width {
                    let (glyph, color) = if filled >= cell_top {
                        // Entire cell is filled — solid block.
                        ("█", cava_bar_color(row as u16, area.height))
                    } else if filled > cell_bottom {
                        // Partial fill at the top of the bar.
                        let frac = filled - cell_bottom;
                        (FRAG[frac.min(7)], cava_bar_color(row as u16, area.height))
                    } else if peak > 0.02
                        && peak_level.saturating_sub(filled) >= 8
                        && peak_level >= cell_bottom
                        && peak_level < cell_top
                    {
                        // Peak cap — thin block at the top of cell.
                        ("▔", RED)
                    } else {
                        (" ", BG)
                    };

                    spans.push(Span::styled(glyph, Style::default().fg(color)));
                }

                // Spacing between bars.
                for _ in 0..bar_spacing {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            // Fill remaining width so the line matches terminal width.
            let used = num_bars * total_per_bar;
            if used < terminal_width {
                for _ in 0..(terminal_width - used) {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            Line::from(spans)
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), area);
}

fn cava_bar_color(row: u16, height: u16) -> Color {
    let ratio_from_bottom = if height <= 1 {
        0.0
    } else {
        1.0 - row as f64 / (height - 1) as f64
    };
    if ratio_from_bottom > 0.82 {
        RED
    } else if ratio_from_bottom > 0.62 {
        YELLOW
    } else if ratio_from_bottom > 0.38 {
        GREEN
    } else {
        TEAL
    }
}

fn mirror_color(row: u16, height: u16) -> Color {
    let ratio = if height <= 1 {
        0.5
    } else {
        row as f64 / (height - 1) as f64
    };
    let dist = (ratio - 0.5).abs() * 2.0;
    if dist < 0.3 {
        SAPPHIRE // center — cool blue
    } else if dist < 0.6 {
        TEAL // green-teal
    } else if dist < 0.8 {
        YELLOW // warm
    } else {
        RED // edges — hot
    }
}

fn interpolate_signal(signal: &[f64], position: usize, width: usize) -> f64 {
    if signal.len() == 1 || width <= 1 {
        return signal[0];
    }
    let source = position as f64 * (signal.len() - 1) as f64 / (width - 1) as f64;
    let left = source.floor() as usize;
    let right = source.ceil().min((signal.len() - 1) as f64) as usize;
    let blend = source - left as f64;
    signal[left] * (1.0 - blend) + signal[right] * blend
}

pub struct App {
    all_items: Vec<MediaItem>,
    filtered_indices: Vec<usize>,
    state: ListState,
    filter: String,
}

impl App {
    pub fn new(items: Vec<MediaItem>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        let filtered_indices = (0..items.len()).collect();
        Self {
            all_items: items,
            filtered_indices,
            state,
            filter: String::new(),
        }
    }

    fn apply_filter(&mut self) {
        let query = self.filter.to_lowercase();
        self.filtered_indices = if query.is_empty() {
            (0..self.all_items.len()).collect()
        } else {
            self.all_items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.title.to_lowercase().contains(&query))
                .map(|(index, _)| index)
                .collect()
        };
        if self.filtered_indices.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.filtered_indices.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_indices.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&MediaItem> {
        self.state
            .selected()
            .and_then(|index| self.filtered_indices.get(index))
            .and_then(|index| self.all_items.get(*index))
    }
}

pub fn run_tui(items: Vec<MediaItem>) -> Result<Option<MediaItem>> {
    let _cleanup = TerminalCleanup::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(items);
    let result = run_app(&mut terminal, &mut app);

    terminal.show_cursor()?;

    result
}

pub fn run_action() -> Result<String> {
    let actions = [
        ("Replay", "Watch the current item again", "replay"),
        ("Next", "Continue with the next item", "next"),
        ("Previous", "Return to the previous item", "previous"),
        ("Choose another", "Open the current result list", "select"),
        ("Back", "Return to the previous screen", "quit"),
    ]
    .into_iter()
    .map(|(title, description, action)| MediaItem {
        title: format!("{:<16} {}", title, description),
        url: action.to_owned(),
        duration: None,
        episode: None,
    })
    .collect();

    Ok(run_tui(actions)?
        .map(|item| item.url)
        .unwrap_or_else(|| "quit".to_owned()))
}

pub fn run_input(title: &str, label: &str, hint: &str) -> Result<Option<String>> {
    let _cleanup = TerminalCleanup::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_input_app(&mut terminal, title, label, hint);
    terminal.show_cursor()?;
    result
}

pub fn run_notice(title: &str, message: &str) -> Result<()> {
    let _cleanup = TerminalCleanup::enter()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_notice_app(&mut terminal, title, message);
    terminal.show_cursor()?;
    result
}

fn run_notice_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    title: &str,
    message: &str,
) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let height = area.height.saturating_sub(2).min(14);
            let width = area.width.saturating_sub(2).min(78);
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(height),
                    Constraint::Min(0),
                ])
                .split(area);
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(width),
                    Constraint::Min(0),
                ])
                .split(vertical[1]);
            let content = Paragraph::new(vec![
                Line::from(Span::styled(message, Style::default().fg(TEXT))),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Enter ", Style::default().fg(BLUE)),
                    Span::styled("return", Style::default().fg(MUTED)),
                ]),
            ])
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(format!(" {} ", title))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(RED)),
            );
            frame.render_widget(content, horizontal[1]);
        })?;

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                return Ok(());
            }
            if matches!(code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q')) {
                return Ok(());
            }
        }
    }
}

fn run_input_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    title: &str,
    label: &str,
    hint: &str,
) -> Result<Option<String>> {
    let mut input = String::new();
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Length(9),
                    Constraint::Min(0),
                ])
                .split(area);
            let width = area.width.saturating_sub(4).min(76);
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(width),
                    Constraint::Min(0),
                ])
                .split(vertical[1]);
            let card = horizontal[1];
            let shown = if input.is_empty() {
                hint
            } else {
                input.as_str()
            };
            let shown_style = if input.is_empty() {
                Style::default().fg(OVERLAY)
            } else {
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
            };
            let content = Paragraph::new(vec![
                Line::from(Span::styled(
                    label,
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        " › ",
                        Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(shown, shown_style),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    " Type a title, channel, URL, or local path",
                    Style::default().fg(MUTED),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" Enter ", Style::default().fg(BLUE)),
                    Span::styled("continue   ", Style::default().fg(MUTED)),
                    Span::styled(" Esc ", Style::default().fg(PINK)),
                    Span::styled("cancel", Style::default().fg(MUTED)),
                ]),
            ])
            .block(
                Block::default()
                    .title(format!(" {} ", title.to_uppercase()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(ROSE))
                    .style(Style::default().bg(SURFACE)),
            );
            frame.render_widget(content, card);
            if !input.is_empty() {
                let cursor_offset =
                    input.chars().count().min(width.saturating_sub(7) as usize) as u16;
                frame.set_cursor(card.x + 4 + cursor_offset, card.y + 3);
            }
        })?;

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            if modifiers.contains(KeyModifiers::ALT) {
                return Ok(None);
            }
            if modifiers.contains(KeyModifiers::CONTROL) {
                match code {
                    KeyCode::Char('c') => return Ok(None),
                    KeyCode::Char('u') => input.clear(),
                    _ => {}
                }
                continue;
            }
            match code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => return Ok(Some(input.trim().to_owned())),
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Char(character) => input.push(character),
                _ => {}
            }
        }
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<Option<MediaItem>> {
    loop {
        terminal.draw(|f| {
            let area = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(area);

            let title_text = Line::from(vec![
                Span::styled(
                    " ◆ ",
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "MEDIA BROWSER",
                    Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("{} items", app.filtered_indices.len()),
                    Style::default().fg(MUTED),
                ),
            ]);
            let title = Paragraph::new(title_text)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(OVERLAY)),
                );
            f.render_widget(title, chunks[0]);

            let filter_text = if app.filter.is_empty() {
                Line::from(vec![
                    Span::styled(" / ", Style::default().fg(BLUE)),
                    Span::styled("Type to filter...", Style::default().fg(OVERLAY)),
                ])
            } else {
                Line::from(vec![
                    Span::styled(" / ", Style::default().fg(ROSE)),
                    Span::styled(
                        &app.filter,
                        Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("_", Style::default().fg(ROSE)),
                ])
            };
            let filter_bar = Paragraph::new(filter_text).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(OVERLAY))
                    .title(Span::styled(" FILTER ", Style::default().fg(BLUE))),
            );
            f.render_widget(filter_bar, chunks[1]);

            let selected_idx = app.state.selected().unwrap_or(0);
            let items: Vec<ListItem> = app
                .filtered_indices
                .iter()
                .enumerate()
                .map(|(idx, item_index)| {
                    let item = &app.all_items[*item_index];
                    let num = format!("{:3}. ", idx + 1);
                    let title = item.title.as_str();
                    let dur_fmt = match &item.duration {
                        Some(d) => format!(" [{}]", d),
                        None => String::new(),
                    };

                    if idx == selected_idx {
                        let content = Line::from(vec![
                            Span::styled(
                                num,
                                Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                title,
                                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(dur_fmt, Style::default().fg(BLUE)),
                        ]);
                        ListItem::new(content)
                    } else {
                        let content = Line::from(vec![
                            Span::styled(num, Style::default().fg(OVERLAY)),
                            Span::styled(title, Style::default().fg(TEXT)),
                            Span::styled(dur_fmt, Style::default().fg(MUTED)),
                        ]);
                        ListItem::new(content)
                    }
                })
                .collect();

            let items_list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .border_style(Style::default().fg(OVERLAY))
                        .title(Span::styled(
                            " RESULTS ",
                            Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
                        )),
                )
                .highlight_style(Style::default().bg(SURFACE1).add_modifier(Modifier::BOLD))
                .highlight_symbol("┃");

            f.render_stateful_widget(items_list, chunks[2], &mut app.state);

            let help = Paragraph::new(Line::from(vec![
                Span::styled(
                    " ↑↓",
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" navigate  ", Style::default().fg(MUTED)),
                Span::styled(
                    "Enter",
                    Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" select  ", Style::default().fg(MUTED)),
                Span::styled("/", Style::default().fg(BLUE).add_modifier(Modifier::BOLD)),
                Span::styled(" filter  ", Style::default().fg(MUTED)),
                Span::styled(
                    "Esc",
                    Style::default().fg(ROSE).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" quit", Style::default().fg(MUTED)),
            ]))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .border_style(Style::default().fg(OVERLAY)),
            );
            f.render_widget(help, chunks[3]);
        })?;

        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                return Ok(None);
            }
            match code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => {
                    if let Some(item) = app.selected() {
                        return Ok(Some(item.clone()));
                    }
                }
                KeyCode::Backspace => {
                    app.filter.pop();
                    app.apply_filter();
                }
                KeyCode::Char(c) => {
                    if c != 'q' || !app.filter.is_empty() {
                        app.filter.push(c);
                        app.apply_filter();
                    } else {
                        return Ok(None);
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(title: &str) -> MediaItem {
        MediaItem {
            title: title.to_owned(),
            url: title.to_owned(),
            duration: None,
            episode: None,
        }
    }

    #[test]
    fn filtering_keeps_only_matching_indices() {
        let mut app = App::new(vec![item("One Piece"), item("Naruto"), item("Bleach")]);
        app.filter = "ARU".to_owned();
        app.apply_filter();

        assert_eq!(app.filtered_indices, vec![1]);
        assert_eq!(
            app.selected().map(|item| item.title.as_str()),
            Some("Naruto")
        );
    }

    #[test]
    fn empty_filter_restores_all_items() {
        let mut app = App::new(vec![item("One"), item("Two")]);
        app.filter = "missing".to_owned();
        app.apply_filter();
        app.filter.clear();
        app.apply_filter();

        assert_eq!(app.filtered_indices, vec![0, 1]);
        assert_eq!(app.state.selected(), Some(0));
    }

    #[test]
    fn navigation_wraps_in_both_directions() {
        let mut app = App::new(vec![item("One"), item("Two")]);
        app.previous();
        assert_eq!(app.state.selected(), Some(1));
        app.next();
        assert_eq!(app.state.selected(), Some(0));
    }
}
