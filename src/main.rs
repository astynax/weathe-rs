extern crate docopt;
extern crate hyper;
extern crate xml;

use std::borrow::Borrow;
use std::fmt;
use std::io::Read;

use docopt::Docopt;

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::status::StatusCode;

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::reader::events::XmlEvent;
use xml::reader;


static USAGE: &'static str = "
Usage: weathe-rs [-f] [<city_id>]
       weathe-rs -h

Options:
    -h, --help         Show this message
    -f, --fahrenheits  Show the temperature in the degrees of the Fahrenheit
                       (instead of the Celsius)
";

static DEFAULT_CITY: &'static str = "2121267"; // Kazan' (Russia)


// --------------------- Data Types ----------------------------------

enum TempUnit {
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

// -------------------------------------------------------------------


fn request_weather(city: String, unit: TempUnit) -> Option<Response> {
    let mut client = Client::new();
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
            Some(WeatherInfo { status: s,
                               degrees: d,
                               unit: u }),
        _ => None
    }
}


fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|d| { d.parse() })
        .unwrap_or_else(|e| { e.exit() });

    let units =
        if args.get_bool("-f") {
            TempUnit::Fahrenheit
        } else {
            TempUnit::Celsius
        };

    let city = ({
        let arg = args.get_str("<city_id>");
        if arg == "" { DEFAULT_CITY } else { arg }
    }).to_string();

    request_weather(city, units)
        .and_then(parse_weather)
        .map_or_else(|| { println!("Oops!") },
                     |w| { println!("{}", w) })
}
