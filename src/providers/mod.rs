mod yahoo;

use types::WeatherProvider;
use providers::yahoo::*;

pub fn get_provider(_name: String) -> Option<WeatherProvider> {
    Some(yahoo_weather)
}
