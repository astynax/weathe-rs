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
        write!(f, "{}", match *self {
            TempUnit::Celsius    => "°C",
            TempUnit::Fahrenheit => "°F"
        })
    }
}

/// Weather Forecast information
pub struct WeatherInfo {
    status: String,
    degrees: i8,
    unit: TempUnit
}

impl WeatherInfo {
    pub fn new(status: String, degrees: i8, unit: TempUnit) -> WeatherInfo {
        WeatherInfo { status: status,
                      degrees: degrees,
                      unit: unit }
    }
}

impl fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}, {}", self.degrees, self.unit, self.status)
    }
}

/// Configuration of the weather forecast
pub struct Configuration {
    city: Option<String>,
    units: Option<TempUnit>,
}

impl Configuration {
    pub fn new(city: Option<String>, units: Option<TempUnit>) -> Configuration {
        Configuration { city: city,
                        units: units }
    }

    pub fn apply(&self, other: Configuration) -> Configuration {
        Configuration {
            city: other.city.or(self.city.clone()),
            units: other.units.or(self.units.clone())
        }
    }

    pub fn unwrap(&self) -> (String, TempUnit) {
        let city = self.city.clone().expect("No CityID configured!");
        let units = self.units.clone().expect("No temp unit configured!");
        (city, units)
    }
}
