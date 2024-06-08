# weact-studio-epd

Unofficial Rust driver for WeAct Studio e-paper displays.

Currently only supports the WeAct Studio 2.9 inch B/W display but support for the other versions is planned.

Supports partial and quick refreshes on the B/W displays.

## Features

- `graphics`: Enables `embedded-graphics` support. Enabled by default.

## Examples

See the `examples` folder for usage examples.

## Credits

This driver is based on the following crates:

- [`epd-waveshare`](https://crates.io/crates/epd-waveshare)
- [`ssd1680`](https://crates.io/crates/ssd1680)

I do not understand software licenses. If you are the author of one of the above crates and you think that
this crate should be licensed under a different license, please let me know.

## License

This crate is licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
