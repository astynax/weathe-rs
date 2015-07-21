//! # CmdLine and config-file handling stuff

use std;
use std::borrow::Borrow;
use std::io::Read;
use std::io::{Error, ErrorKind};

use types::Configuration;
use types::TempUnit;

extern crate docopt;
extern crate toml;


static USAGE: &'static str = "
Usage: weathe_rs [-f] [<provider_id>] [<city_id>]
       weathe_rs -h

Options:
    -h, --help         Show this message
    -f, --fahrenheits  Show the temperature in the degrees of the Fahrenheit
                       (instead of the Celsius)
";


pub fn get_options() -> Configuration {
    let args = docopt::Docopt::new(USAGE)
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

    let provider = {
        let arg = args.get_str("<provider_id>");
        if arg == "" { None } else { Some(arg.to_string()) }
    };

    Configuration::new(city, units, provider)
}


pub fn get_config() -> Configuration {
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
                        let provider = match params.get("provider") {
                            Some(&toml::Value::String(ref s)) =>
                                Some(s.clone()),
                            _ => None
                        };
                        Some(Configuration::new(city, units, provider))
                    })
                    .ok_or(Error::new(ErrorKind::Other,
                                      "Can't parse the config"))
            })
        }).unwrap_or_else(|e| {
            if e.raw_os_error() != Some(2) { // file not found
                panic!("Config parsing error: {:?}\n", e)
            } else {
                Configuration::new(None, None, None)
            }
        })
}
