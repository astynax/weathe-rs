//! # Weather provider using *OpenWeatherMaps*

use std::io::Read;

extern crate serde;

use self::serde::json;

use types::{WeatherInfo, WeatherResult, TempUnit};
use providers::request;

pub fn owm_weather(city: String, units: TempUnit) -> WeatherResult {
    let mut content = String::new();
    if request(
        "http://api.openweathermap.org/data/2.5/weather".to_string()
            + "?id=" + &city
            + "&units=" + match units {
                TempUnit::Celsius => "metric",
                TempUnit::Fahrenheit => "imperial"
            },
        &mut content) {
        println!("{}", content);
        Err("Baz!".to_string())
    } else {
        Err("Foo!".to_string())
    }
}
