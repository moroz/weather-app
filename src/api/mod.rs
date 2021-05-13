extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Error, ErrorKind, Result};
use url::form_urlencoded;

#[derive(Serialize, Deserialize)]
pub struct WeatherDataMain {
    temp: f64,
    temp_max: f64,
    temp_min: f64,
    feels_like: f64,
    pressure: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    name: String,
    main: WeatherDataMain,
    coord: Coordinates,
}

pub const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const ABSOLUTE_ZERO: f64 = 273.15;

fn convert_weather(kelvin: f64) -> f64 {
    kelvin - ABSOLUTE_ZERO
}

fn get_token() -> Result<String> {
    match env::var("WEATHER_API_TOKEN") {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(ErrorKind::NotFound, "Could not fetch API token")),
    }
}

fn encode_url_for_city(city: &str) -> Result<String> {
    let token = get_token()?;

    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("q", city)
        .append_pair("appid", &token)
        .finish();

    let joined = format!("{}?{}", BASE_URL, encoded);

    Ok(joined)
}

pub fn parse_weather_data(json: &str) -> Option<WeatherData> {
    serde_json::from_str(json).ok()
}

pub async fn get_weather_for_city(city: &str) -> Result<String> {
    let url = encode_url_for_city(city)?;

    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    Ok(body)
}

pub async fn get_formatted_weather_for_city(city: &str) -> Result<String> {
    let json = get_weather_for_city(city).await?;
    println!("Original JSON:\n{}", json);

    match parse_weather_data(&json) {
        Some(parsed) => {
            let formatted = format!(
                "Current weather for {}:\n{:.1}",
                parsed.name,
                convert_weather(parsed.main.temp)
            );

            Ok(formatted)
        }
        None => Err(Error::new(ErrorKind::Other, "Invalid JSON")),
    }
}
