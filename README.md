# Yahoo Weather CLI-ent

[![Clippy Linting Result](https://clippy.bashy.io/github/astynax/weathe-rs/master/badge.svg)](https://clippy.bashy.io/github/astynax/weathe-rs/master/log)

**Rust-lang** powered *CLI*-client for *Yahoo Weather API*

### Features:

- Main functionality (requesting of weather forecast)
- CLI-options
- Configuration file
- Data sources
    - [Yahoo Weather API](https://developer.yahoo.com/weather/) (doesn't work anymore, need some actualization :( )
    - [OpenWeatherMap](http://openweathermap.org/api)

### Installation and usage

```sh
$ cargo build --release
    Compiling ...
...

$ target/release/weathe_rs owm 524901
21°C, Fair  # or smth like this

$ target/release/weathe_rs --help
Usage: weathe-rs ...
...
```

### OS-level requirements (build time only)

```sh
$ sudo apt-get install libssl-dev
```

