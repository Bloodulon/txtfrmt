use crate::settings::Settings;
use crate::transform::Transform;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io::{self, stdout};

pub fn run_settings_ui(settings: &mut Settings) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let result = ui_loop(&mut terminal, settings);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    result
}

fn ui_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, settings: &mut Settings) -> io::Result<()> {
    let mut selected = 0usize;
    let transforms: Vec<Transform> = vec![
        Transform::ToRussian,
        Transform::ToEnglish,
        Transform::ToggleLayout,
        Transform::ToggleCase,
        Transform::ToUpper,
        Transform::ToLower,
        Transform::TitleCase,
        Transform::CleanWhitespace,
        Transform::Reverse,
        Transform::ToCamelCase,
        Transform::ToSnakeCase,
    ];

    loop {
        terminal.draw(|f| draw(f, settings, selected, &transforms))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    settings.save();
                    break Ok(());
                }
                KeyCode::Up => {
                    if selected > 0 { selected -= 1; }
                }
                KeyCode::Down => {
                    if selected < transforms.len() - 1 { selected += 1; }
                }
                KeyCode::Enter => {
                    let t = transforms[selected];
                    toggle_hotkey(settings, t);
                }
                _ => {}
            }
        }
    }
}

fn draw(frame: &mut Frame, settings: &Settings, selected: usize, transforms: &[Transform]) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .split(area);

    let title = Paragraph::new("textfmt — Settings")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    frame.render_widget(title, chunks[0]);

    let header = Row::new(vec!["Transform", "Hotkey"])
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);

    let rows: Vec<Row> = transforms.iter().enumerate().map(|(i, t)| {
        let hk = find_hotkey(settings, *t);
        let hotkey_str = hk.map(|h| format!("{}+{}", h.modifiers.join("+"), h.key)).unwrap_or_else(|| "—".to_string());

        let style = if i == selected {
            Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(t.name()),
            Cell::from(hotkey_str),
        ]).style(style).height(1)
    }).collect();

    let table = Table::new(rows, vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .header(header)
        .block(Block::default().title(" Hotkeys ").borders(Borders::ALL));
    frame.render_widget(table, chunks[1]);

    let help = Paragraph::new(vec![
        Line::from("↑/↓ — navigate  |  Enter — toggle hotkey  |  q — save & quit"),
        Line::from(Span::styled(
            format!("Config: {}", Settings::config_path().display()),
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}

fn find_hotkey(settings: &Settings, transform: Transform) -> Option<&crate::settings::HotkeyConfig> {
    settings.hotkeys.iter().find(|h| h.transform == transform)
}

fn toggle_hotkey(settings: &mut Settings, transform: Transform) {
    let exists = settings.hotkeys.iter().position(|h| h.transform == transform);
    if let Some(idx) = exists {
        settings.hotkeys.remove(idx);
    } else {
        settings.hotkeys.push(crate::settings::HotkeyConfig {
            modifiers: vec!["CONTROL".to_string(), "SHIFT".to_string()],
            key: "KeyF".to_string(),
            transform,
        });
    }
}
