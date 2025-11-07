use anyhow::{anyhow, Result};
use chrono::{Local};
use reqwest::Client;
use serde::Deserialize;

use crate::app::WeatherInfo;

#[derive(Debug, Deserialize)]
struct WeatherResp {
    name: String,
    weather: Vec<WeatherDesc>,
    main: Main,
    wind: Wind,
}

#[derive(Debug, Deserialize)]
struct WeatherDesc {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: u64,
    pressure: u64,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}

pub struct WeatherClient {
    client: Client,
    api_key: String,
    units: String, // "metric" or "imperial"
}

impl WeatherClient {
    pub fn new(api_key: String, units: &str) -> Self {
        Self {
            client: Client::new(),
            api_key,
            units: units.to_string(),
        }
    }

    pub async fn fetch_city(&self, city: &str) -> Result<WeatherInfo> {
        let url = "https://api.openweathermap.org/data/2.5/weather";
        let resp = self
            .client
            .get(url)
            .query(&[("q", city), ("appid", &self.api_key), ("units", &self.units)])
            .send()
            .await?
            .error_for_status()?
            .json::<WeatherResp>()
            .await?;

        let description = resp
            .weather
            .get(0)
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "n/a".to_string());

        Ok(WeatherInfo {
            name: resp.name,
            temp_c: resp.main.temp,
            humidity: resp.main.humidity,
            pressure: resp.main.pressure,
            wind_speed: resp.wind.speed,
            description,
            updated_at: Local::now(),
        })
    }
}

/// Read API key from env. Safer than hardcoding.
pub fn load_api_key() -> Result<String> {
    std::env::var("OPENWEATHER_API_KEY")
        .map_err(|_| anyhow!("Set OPENWEATHER_API_KEY in your environment"))
}