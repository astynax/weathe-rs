//! # Weather provider using *Yahoo Weather API*

extern crate xml;

use std::borrow::Borrow;

use self::xml::attribute::OwnedAttribute;
use self::xml::name::OwnedName;
use self::xml::reader::events::XmlEvent;
use self::xml::reader;

use types::{TempUnit, WeatherInfo, WeatherResult};
use providers::request;


fn parse_weather(xml: &String) -> Option<WeatherInfo> {

    fn get_attr(name: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
        attrs.iter()
            .find(|&att| {
                let OwnedAttribute { name: OwnedName { local_name: ref n, ..},
                                     ..} = *att;
                n == name
            })
            .map(|a| { a.value.clone() })
    }

    let mut rdr = reader::EventReader::from_str((*xml).borrow());

    let mut condition: Option<(String, i8)> = None;
    let mut unit: Option<TempUnit> = None;

    loop {
        match rdr.next() {
            XmlEvent::StartElement {
                name: OwnedName { local_name: ref n, .. },
                attributes: ref atts,
                ..} =>
                match n.borrow() {
                    "units" => {
                        unit = get_attr("temperature", atts).map(|u| {
                            match u.borrow() {
                                "C" => TempUnit::Celsius,
                                "F" => TempUnit::Fahrenheit,
                                _  => panic!("Unknown TempUnit!")
                            }
                        })},
                    "condition" => {
                        condition = match (
                            get_attr("text", atts),
                            get_attr("temp", atts).map(|t| {t.parse()})) {
                            (Some(s), Some(Ok(t))) => Some((s, t)),
                            _ => None
                        }
                    },
                    _ => ()
                },
            XmlEvent::EndDocument | XmlEvent::Error(..) => break,
            _ => ()
        }
    }

    match (condition, unit) {
        (Some((s, d)), Some(u)) =>
            Some(WeatherInfo::new(s, d, u)),
        _ => None
    }
}


/// Public ``WeatherProvider`` for Yahoo Weather API
pub fn yahoo_weather(city: String, units: TempUnit) -> WeatherResult {
    let mut content = String::new();
    if request(
        "http://weather.yahooapis.com/forecastrss".to_string()
            + "?w=" + &city
            + "&u=" + match units {
                TempUnit::Celsius    => "c",
                TempUnit::Fahrenheit => "f"
            },
        &mut content) {
        parse_weather(&content)
            .ok_or("Can't get info from Yahoo API!".to_string())
    } else {
        Err("Can't request the data!".to_string())
    }
}
