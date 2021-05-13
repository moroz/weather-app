extern crate dotenv;
extern crate futures;

pub mod api;

use dotenv::dotenv;
use std::env;
use tokio::runtime;

fn print_usage() {
    println!("Weather app. Usage: weather CITY")
}

fn main() {
    dotenv().ok();

    let argc = env::args().len();

    if argc == 1 {
        print_usage();
    } else {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let city = env::args().nth(1).unwrap();
        println!("Selected city: {}", city);
        let future = async {
            let response = match api::get_weather_for_city(&city).await {
                Ok(weather) => weather,
                Err(_) => "Failed to fetch weather.".to_string(),
            };
            println!("{}", response);
        };
        rt.block_on(future);
    }
}
