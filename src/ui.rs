use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::{App};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)])
        .split(frame.size());

    draw_header(frame, chunks[0]);
    draw_body(frame, chunks[1], app);
    draw_footer(frame, chunks[2], &app.status);
}

fn draw_header(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" Weather CLI (Ratatui + OpenWeather) ", Style::default().add_modifier(Modifier::BOLD)),
    ]))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn draw_body(frame: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    // Cities list
    let items: Vec<ListItem> = app
        .cities
        .iter()
        .enumerate()
        .map(|(i, city)| {
            let marker = if i == app.selected { "▶ " } else { "  " };
            ListItem::new(Line::from(vec![Span::raw(format!("{marker}{city}"))]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Cities")
            .borders(Borders::ALL),
    );

    frame.render_widget(list, columns[0]);

    // Weather Details
    let details_block = Block::default().title("Weather Details").borders(Borders::ALL);
    let detail_text = if let Some(city) = app.selected_city() {
        if let Some(info) = app.get_weather(city) {
            vec![
                Line::raw(format!("City: {}", info.name)),
                Line::raw(format!("Temp: {:.1}°F", info.temp_c)),
                Line::raw(format!("Humidity: {}%", info.humidity)),
                Line::raw(format!("Pressure: {} hPa", info.pressure)),
                Line::raw(format!("Wind: {:.1} m/s", info.wind_speed)),
                Line::raw(format!("Conditions: {}", info.description)),
                Line::raw(format!("Last updated: {}", info.updated_at.format("%Y-%m-%d %H:%M:%S"))),
                Line::raw(""),
                Line::raw("Tips: Press R to refresh; ↑/↓ to switch city; Q to quit."),
            ]
        } else {
            vec![
                Line::raw(format!("Selected: {city}")),
                Line::raw("No data cached. Press R to fetch."),
            ]
        }
    } else {
        vec![Line::raw("No city selected")]
    };

    let details = Paragraph::new(detail_text).wrap(Wrap { trim: true }).block(details_block);
    frame.render_widget(details, columns[1]);
}

fn draw_footer(frame: &mut Frame, area: Rect, status: &str) {
    let footer = Paragraph::new(status.to_string())
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, area);
}