use std::time::Duration;

use anyhow::Result;
use crossterm::event::KeyEventKind;
use weather_cli::{
    app::App,
    connection::{load_api_key, WeatherClient},
    event::{Event, Events},
    handler::handle_key_events,
    tui::Tui,
    ui::draw,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Cities you want to browse
    let cities = vec![
        "Bucharest".to_string(),
        "Craiova".to_string(),
        "London".to_string(), 
        "New York".to_string(),
        "Tokyo".to_string(),
    ];

    // Initialize state and API client
    let mut app = App::new(cities);
    let api_key = load_api_key()?;
    let client = WeatherClient::new(api_key, "metric");

    // Prime cache for first city (optional, but nice UX)
    if let Some(city) = app.selected_city().map(|s| s.to_string()) {
        if let Ok(info) = client.fetch_city(&city).await {
            app.put_weather(&city, info);
            app.set_status("Fetched initial city. R to refresh, Q to quit.");
        }
    }

    // Terminal + events
    let mut tui = Tui::new()?;
    let mut events = Events::new(Duration::from_millis(500));

    // Main event/render loop
    loop {
        tui.draw(|f| draw(f, &app))?;

        match events.next().await {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                let before = app.selected;
                let was_r = matches!(key.code, crossterm::event::KeyCode::Char('r'));

                // Update selection / quit flags, etc.
                handle_key_events(key, &mut app)?;

                let selection_changed = app.selected != before;
                if !app.running {
                    break;
                }

                // Refresh the selected city if selection changed or user pressed 'r'
                if selection_changed || was_r {
                    if let Some(city) = app.selected_city().map(|s| s.to_string()) {
                        app.set_status(format!("Fetching {city}…"));
                        match client.fetch_city(&city).await {
                            Ok(info) => {
                                app.put_weather(&city, info);
                                app.set_status(format!("Updated {city}."));
                            }
                            Err(e) => {
                                app.set_status(format!("Error: {e}"));
                            }
                        }
                    }
                }
            }
            Some(Event::Tick) => {
                // Optional: implement auto-refresh here (e.g., every N ticks)
            }
            _ => {}
        }

        if !app.running {
            break;
        }
    }

    // Tui Drop restores terminal
    Ok(())
}