# Yahoo Weather CLI-ent

**Rust-lang** powered *CLI*-client for *Yahoo Weather API*

### Features:

- Main functionality (requesting of weather forecast)
- CLI-options
- Configuration file
- **TODO** Other data sources

### Installation and usage

```sh
$ cargo build --release
    Compiling ...
...

$ target/release/weathe-rs
21°C, Fair  # or smth like this

$ target/release/weathe-rs --help
Usage: weathe-rs ...
...
```

### OS-level requirements (build time only)

```sh
$ sudo apt-get install libssl-dev
```

