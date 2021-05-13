extern crate reqwest;
extern crate url;

use std::env;
use std::io::{Error, ErrorKind, Result};
use url::form_urlencoded;

pub const BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

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

pub async fn get_weather_for_city(city: &str) -> Result<String> {
    let url = encode_url_for_city(city)?;

    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    Ok(body)
}
