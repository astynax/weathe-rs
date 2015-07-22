mod yahoo;
mod owm;

use std::io::Read;
use std::borrow::Borrow;

extern crate hyper;

use self::hyper::client::Client;
use self::hyper::status::StatusCode;

use types::WeatherProvider;
use providers::yahoo::yahoo_weather;
use providers::owm::owm_weather;


/// Returns a WeatherProvider by name
pub fn get_provider(name: String) -> Option<WeatherProvider> {
    match name.borrow() {
        "yahoo" => Some(yahoo_weather),
        "owm" => Some(owm_weather),
        _ => None
    }
}

/// Sends a *GET*-request
pub fn request(url: String, out: &mut String) -> bool {
    let client = Client::new();
    let res = client.get(&url).send().ok();
    res.map(|mut res| {
        if res.status == StatusCode::Ok {
            res.read_to_string(out).is_ok()
        } else {
            false
        }
    }).unwrap_or(false)
}
