extern crate weathe_rs;

use std::process;

use weathe_rs::types::{Configuration, TempUnit};
use weathe_rs::environ;
use weathe_rs::providers::{get_provider};


fn main() {
    match Configuration::new(
        None, Some(TempUnit::Celsius), Some("yahoo".to_string()))
        .apply(environ::get_config())
        .apply(environ::get_options())
        .get_weather_by(get_provider)
    {
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        },
        Ok(w) => println!("{}", w)
    }
}
