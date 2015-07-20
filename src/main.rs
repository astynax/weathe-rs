extern crate docopt;
extern crate hyper;
extern crate xml;
extern crate toml;

use std::borrow::Borrow;
use std::fmt;
use std::io::Read;
use std::io::{Error, ErrorKind};

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
#[derive(Clone)]
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


struct Configuration {
    city: Option<String>,
    units: Option<TempUnit>,
}

impl Configuration {
    fn or(&self, other: Configuration) -> Configuration {
        Configuration {
            city: other.city.or(self.city.clone()),
            units: other.units.or(self.units.clone())
        }
    }

    fn unwrap(&self) -> (String, TempUnit) {
        (self.city.clone().unwrap(), self.units.clone().unwrap())
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


fn get_options() -> Configuration {
    let args = Docopt::new(USAGE)
        .and_then(|d| { d.parse() })
        .unwrap_or_else(|e| { e.exit() });

    let units =
        if args.get_bool("-f") {
            Some(TempUnit::Fahrenheit)
        } else {
            None
        };

    let city = {
        let arg = args.get_str("<city_id>");
        if arg == "" { None } else { Some(arg.to_string()) }
    };

    Configuration { city: city, units: units }
}


fn get_config() -> Configuration {
    let home = std::env::home_dir().expect("Can't get $HOME");

    let cfg_path = home.as_path().join(".config").join(".weathe-rs");

    std::fs::File::open(cfg_path)
        .and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content).and_then(|_| {
                toml::Parser::new(content.borrow())
                    .parse()
                    .and_then(|root| {
                        match root.get("params") {
                            Some(&toml::Value::Table(ref t)) =>
                                Some(t.clone()),
                            _ => None
                        }
                    })
                    .and_then(|params| {
                        let city = match params.get("city") {
                            Some(&toml::Value::Integer(ref s)) =>
                                Some(s.to_string()),
                            _ => None
                        };
                        let units = match params.get("fahrenheits") {
                            Some(&toml::Value::Boolean(ref b)) =>
                                Some(if *b {
                                    TempUnit::Fahrenheit
                                } else {
                                    TempUnit::Celsius }),
                            _ => None
                        };
                        Some(Configuration { city: city, units: units })
                    })
                    .ok_or(Error::new(ErrorKind::Other,
                                      "Can't parse the config"))
            })
        }).unwrap_or_else(|e| {
            if e.raw_os_error() != Some(2) { // file not found
                panic!("Config parsing error: {:?}\n", e)
            } else {
                Configuration { city: None, units: None }
            }
        })
}


fn main() {
    let (city, units) = Configuration {
        city: Some(DEFAULT_CITY.to_string()),
        units: Some(TempUnit::Celsius)}
        .or(get_config())
        .or(get_options())
        .unwrap();

    request_weather(city, units)
        .and_then(parse_weather)
        .map_or_else(|| { println!("Oops!") },
                     |w| { println!("{}", w) })
}
