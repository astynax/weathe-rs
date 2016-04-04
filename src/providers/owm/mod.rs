//! # Weather provider using *OpenWeatherMaps*

use std::borrow::Borrow;

extern crate rustc_serialize;

use self::rustc_serialize::json::Json;

use types::{WeatherInfo, WeatherResult, TempUnit};
use providers::request;


fn parse_weather(raw: String, units: TempUnit) -> WeatherResult {
    Json::from_str(raw.borrow())
        .ok()
        .and_then(|j| {
            let temperature = j.find_path(&["main", "temp"]).and_then(|t| t.as_f64());
            let description = j.as_object().and_then(|o| {
                o.get("weather").and_then(|j| {
                    j.as_array().and_then(|a| {
                        a[0].as_object()
                            .and_then(|o| o.get("description").and_then(|d| d.as_string()))
                    })
                })
            });
            description.and_then(|d| {
                temperature.and_then(|t| Some(WeatherInfo::new(d.to_string(), t as i8, units)))
            })
        })
        .ok_or("Foo!".to_string())
}


/// Public ``WeatherProvider`` for OpenWeatherMap
pub fn owm_weather(city: String, units: TempUnit) -> WeatherResult {
    let mut content = String::new();
    if request("http://api.openweathermap.org/data/2.5/weather?lang=ru".to_string() + "&id=" +
               &city + "&units=" +
               match units {
                   TempUnit::Celsius => "metric",
                   TempUnit::Fahrenheit => "imperial",
               },
               &mut content) {
        parse_weather(content, units)
    } else {
        Err("Foo!".to_string())
    }
}
