use std::collections::HashMap;
use chrono::{DateTime, Local};

pub type AppResult<T> = anyhow::Result<T>;

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub name: String,
    pub temp_c: f64,
    pub humidity: u64,
    pub pressure: u64,
    pub wind_speed: f64,
    pub description: String,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub cities: Vec<String>,
    pub selected: usize,
    pub cache: HashMap<String, WeatherInfo>,
    pub status: String,
}

impl App {
    pub fn new(cities: Vec<String>) -> Self {
        Self {
            running: true,
            cities,
            selected: 0,
            cache: HashMap::new(),
            status: "Press R to refresh, ↑/↓ to navigate, Q to quit".into(),
        }
    }

    pub fn selected_city(&self) -> Option<&str> {
        self.cities.get(self.selected).map(|s| s.as_str())
    }

    pub fn next(&mut self) {
        if !self.cities.is_empty() {
            self.selected = (self.selected + 1) % self.cities.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.cities.is_empty() {
            if self.selected == 0 {
                self.selected = self.cities.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status = msg.into();
    }

    pub fn put_weather(&mut self, city: &str, info: WeatherInfo) {
        self.cache.insert(city.to_string(), info);
    }

    pub fn get_weather(&self, city: &str) -> Option<&WeatherInfo> {
        self.cache.get(city)
    }
}