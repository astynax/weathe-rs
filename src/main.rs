extern crate hyper;
extern crate xml;

use std::borrow::Borrow;
use std::io::Read;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::status::StatusCode;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::events::XmlEvent;
use xml::reader;

extern crate weathe_rs;

use weathe_rs::types::{TempUnit, WeatherInfo, Configuration,
                       WeatherResult, WeatherProvider};
use weathe_rs::environ;


fn request_weather(city: String, unit: TempUnit) -> Option<Response> {
    let client = Client::new();
    let res = client.get(
        &("http://weather.yahooapis.com/forecastrss".to_string()
          + "?w=" + &city
          + "&u=" + match unit {
              TempUnit::Celsius    => "c",
              TempUnit::Fahrenheit => "f"
          }))
        .send()
        .ok();
    res.and_then(|res| {
        if res.status == StatusCode::Ok {
            Some(res)
        } else {
            None
        }
    })
}


fn parse_weather<R: Read>(xml: R) -> Option<WeatherInfo> {

    fn get_attr(name: &str, attrs: &Vec<OwnedAttribute>) -> Option<String> {
        attrs.iter()
            .find(|&att| {
                let OwnedAttribute { name: OwnedName { local_name: ref n, ..},
                                     ..} = *att;
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


fn yahoo_weather(c: String, u: TempUnit) -> WeatherResult {
    request_weather(c, u)
        .and_then(parse_weather)
        .ok_or("Oops!".to_string())
}


fn main() {
    fn get_provider(_name: String) -> Option<WeatherProvider> {
        Some(yahoo_weather)
    };
    match Configuration::new(
        None, Some(TempUnit::Celsius), Some("yahoo".to_string()))
        .apply(environ::get_config())
        .apply(environ::get_options())
        .get_weather_by(get_provider)
    {
        Err(e) => println!("Error: {}", e),
        Ok(w) => println!("{}", w)
    }
}
