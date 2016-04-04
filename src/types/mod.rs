//! # Internal datatypes

use std::fmt;


/// Temperature Units
#[derive(Clone)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   TempUnit::Celsius => "°C",
                   TempUnit::Fahrenheit => "°F",
               })
    }
}


/// Weather Forecast information
pub struct WeatherInfo {
    status: String,
    degrees: i8,
    unit: TempUnit,
}

impl WeatherInfo {
    pub fn new(status: String, degrees: i8, unit: TempUnit) -> WeatherInfo {
        WeatherInfo {
            status: status,
            degrees: degrees,
            unit: unit,
        }
    }
}

impl fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}, {}", self.degrees, self.unit, self.status)
    }
}


/// Result of getting of the weather info
pub type WeatherResult = Result<WeatherInfo, String>;

/// Weather data provider
pub type WeatherProvider = fn(String, TempUnit) -> WeatherResult;


/// Configuration of the weather forecast
pub struct Configuration {
    city: Option<String>,
    units: Option<TempUnit>,
    provider: Option<String>,
}

impl Configuration {
    pub fn new(city: Option<String>,
               units: Option<TempUnit>,
               provider: Option<String>)
               -> Configuration {
        Configuration {
            city: city,
            units: units,
            provider: provider,
        }
    }

    pub fn apply(&self, other: Configuration) -> Configuration {
        Configuration {
            city: other.city.or(self.city.clone()),
            units: other.units.or(self.units.clone()),
            provider: other.provider.or(self.provider.clone()),
        }
    }

    pub fn get_weather_by(&self,
                          get_provider: fn(String) -> Option<WeatherProvider>)
                          -> WeatherResult {
        self.provider.clone().map_or(Err("No provider specified!".to_string()), |prov| {
            get_provider(prov)
                .ok_or("Unknown provider!".to_string())
                .and_then(|prov| {
                    self.city.clone().map_or(Err("No city specified!".to_string()), |city| {
                        self.units.clone().map_or(Err("No units specified!".to_string()),
                                                  |units| prov(city, units))
                    })
                })
        })
    }
}
