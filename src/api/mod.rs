use std::env;

fn get_token() -> Option<String> {
    match env::var("WEATHER_API_TOKEN") {
        Ok(val) => Some(val),
        _ => None,
    }
}
