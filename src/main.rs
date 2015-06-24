extern crate hyper;
extern crate xml;

use std::borrow::Borrow;
use std::fmt;
use std::io::Read;

use hyper::client::Client;
use hyper::status::StatusCode;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::events::XmlEvent;
use xml::reader;


enum TempUnit {
    Celsius,
    Farenheit,
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &TempUnit::Celsius   => "°C",
            &TempUnit::Farenheit => "°F"
        })
    }
}


struct WeatherInfo {
    status: String,
    degrees: i8,
    unit: TempUnit
}

impl fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}, {}", self.degrees, self.unit, self.status)
    }
}


fn request_weather(city: String, unit: TempUnit) -> Option<Box<Read>> {
    let mut client = Client::new();
    let res = client.get(
        &("http://weather.yahooapis.com/forecastrss".to_string()
          + "?w=" + &city
          + "&u=" + match unit {
              TempUnit::Celsius   => "c",
              TempUnit::Farenheit => "f"
          }
          )).send().unwrap();

    if res.status == StatusCode::Ok {
        Some(Box::new(res))
    } else {
        None
    }
}


fn parse_weather(xml: Box<Read>) -> Option<WeatherInfo> {

    fn get_attr(name: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
        attrs.iter()
            .find(|&att| {
                let &OwnedAttribute { name: OwnedName { local_name: ref n, ..},
                                      ..} = att;
                n == name
            })
            .map(|a| { a.value.clone() })
    }

    let cfg = reader::config::ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(true);
    let mut rdr = reader::EventReader::with_config(xml, cfg);

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
                                "F" => TempUnit::Farenheit,
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
            Some(WeatherInfo { status: s,
                               degrees: d,
                               unit: u }),
        _ => None
    }
}


fn main() {
    request_weather("2121267".to_string(),
                    TempUnit::Celsius)
        .and_then(parse_weather)
        .map_or_else(|| { println!("Oops!") },
                     |w| { println!("{}", w) })
}
