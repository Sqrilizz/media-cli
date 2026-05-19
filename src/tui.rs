use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, BorderType},
    Terminal,
};
use std::io;

use crate::sources::MediaItem;

pub struct App {
    all_items: Vec<MediaItem>,
    filtered_items: Vec<MediaItem>,
    state: ListState,
    filter: String,
}

impl App {
    pub fn new(items: Vec<MediaItem>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        let filtered = items.clone();
        Self {
            all_items: items,
            filtered_items: filtered,
            state,
            filter: String::new(),
        }
    }

    fn apply_filter(&mut self) {
        let query = self.filter.to_lowercase();
        self.filtered_items = if query.is_empty() {
            self.all_items.clone()
        } else {
            self.all_items
                .iter()
                .filter(|i| i.title.to_lowercase().contains(&query))
                .cloned()
                .collect()
        };
        if self.filtered_items.is_empty() {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.filtered_items.is_empty() { return; }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.filtered_items.len() - 1 { 0 } else { i + 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.filtered_items.is_empty() { return; }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 { self.filtered_items.len() - 1 } else { i - 1 }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&MediaItem> {
        self.state.selected().and_then(|i| self.filtered_items.get(i))
    }
}

pub fn run_tui(items: Vec<MediaItem>) -> Result<Option<MediaItem>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(items);
    let result = run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(result)
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
                Span::styled(" ▶ ", Style::default().fg(Color::Rgb(170, 100, 255)).add_modifier(Modifier::BOLD)),
                Span::styled("MEDIA-CLI", Style::default().fg(Color::Rgb(130, 180, 255)).add_modifier(Modifier::BOLD)),
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("{} items", app.filtered_items.len()),
                    Style::default().fg(Color::Rgb(100, 100, 120)),
                ),
            ]);
            let title = Paragraph::new(title_text)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Rgb(60, 80, 120)))
                );
            f.render_widget(title, chunks[0]);

            let filter_text = if app.filter.is_empty() {
                Line::from(vec![
                    Span::styled(" / ", Style::default().fg(Color::Rgb(100, 100, 120))),
                    Span::styled("Type to filter...", Style::default().fg(Color::Rgb(80, 80, 100))),
                ])
            } else {
                Line::from(vec![
                    Span::styled(" / ", Style::default().fg(Color::Rgb(170, 100, 255))),
                    Span::styled(&app.filter, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    Span::styled("_", Style::default().fg(Color::Rgb(170, 100, 255))),
                ])
            };
            let filter_bar = Paragraph::new(filter_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Rgb(60, 80, 120)))
                        .title(Span::styled(" Filter ", Style::default().fg(Color::Rgb(100, 100, 140))))
                );
            f.render_widget(filter_bar, chunks[1]);

            let selected_idx = app.state.selected().unwrap_or(0);
            let items: Vec<ListItem> = app
                .filtered_items
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let num = format!("{:3}. ", idx + 1);
                    let title = item.title.clone();
                    let dur_fmt = match &item.duration {
                        Some(d) => format!(" [{}]", d),
                        None => String::new(),
                    };

                    if idx == selected_idx {
                        let content = Line::from(vec![
                            Span::styled(num, Style::default().fg(Color::Rgb(170, 100, 255)).add_modifier(Modifier::BOLD)),
                            Span::styled(title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                            Span::styled(dur_fmt, Style::default().fg(Color::Rgb(130, 180, 255))),
                        ]);
                        ListItem::new(content)
                    } else {
                        let content = Line::from(vec![
                            Span::styled(num, Style::default().fg(Color::Rgb(80, 80, 100))),
                            Span::styled(title, Style::default().fg(Color::Rgb(200, 200, 210))),
                            Span::styled(dur_fmt, Style::default().fg(Color::Rgb(80, 100, 130))),
                        ]);
                        ListItem::new(content)
                    }
                })
                .collect();

            let items_list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Rgb(60, 80, 120)))
                        .title(Span::styled(" Results ", Style::default().fg(Color::Rgb(130, 180, 255)).add_modifier(Modifier::BOLD)))
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Rgb(40, 40, 70))
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(" ▸ ");

            f.render_stateful_widget(items_list, chunks[2], &mut app.state);

            let help = Paragraph::new(Line::from(vec![
                Span::styled(" ↑↓", Style::default().fg(Color::Rgb(170, 100, 255)).add_modifier(Modifier::BOLD)),
                Span::styled(" navigate  ", Style::default().fg(Color::Rgb(140, 140, 160))),
                Span::styled("Enter", Style::default().fg(Color::Rgb(100, 220, 120)).add_modifier(Modifier::BOLD)),
                Span::styled(" select  ", Style::default().fg(Color::Rgb(140, 140, 160))),
                Span::styled("/", Style::default().fg(Color::Rgb(130, 180, 255)).add_modifier(Modifier::BOLD)),
                Span::styled(" filter  ", Style::default().fg(Color::Rgb(140, 140, 160))),
                Span::styled("Esc", Style::default().fg(Color::Rgb(255, 100, 100)).add_modifier(Modifier::BOLD)),
                Span::styled(" quit", Style::default().fg(Color::Rgb(140, 140, 160))),
            ]))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Rgb(60, 80, 120)))
            );
            f.render_widget(help, chunks[3]);
        })?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
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
