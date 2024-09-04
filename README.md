# weact-studio-epd

Unofficial Rust driver for WeAct Studio e-paper displays.

The driver exposes both write access to the screen frame buffers and higher-level `embedded-graphics` support.

By default this driver uses `async`. If you prefer to use a blocking API instead you can enable the `blocking` feature.

## Supported displays

| Display | Colors | Supported | Partial update[^1] | Fast refresh[^2] | Tested |
|---|---|:---:|:---:|:---:|:---:|
| WeAct 1.54 inch 200x200 B/W | Black, White | ✕ |  |  |  |
| WeAct 2.13 inch 122x250 B/W | Black, White | ✓ | ✓ | ✓ | ✓ |
| WeAct 2.13 inch 122x250 B/W/R | Black, White, Red | ✓ |  | ✕ |  |
| WeAct 2.9 inch 128x296 B/W | Black, White | ✓ | ✓ | ✓ | ✓ |
| WeAct 2.9 inch 128x296 B/W/R | Black, White, Red | ✓ |  | ✕ | ✓ |
| WeAct 4.2 inch 400x300 B/W | Black, White | ✓ | ✓ | ✓ | ✓ |

[^1]: Allows updating part of the screen buffer to save IO time and potentially memory.

[^2]: Refresh the screen without flickering the screen a few times.

## Examples

See the `examples` folder for complete usage examples.

## Features

- `blocking`: Replaces the API with a blocking version. This disables the `async` API so you cannot use both in the same project.
- `graphics`: Enables `embedded-graphics` support. Enabled by default.

## Credits

This driver is based on the following crates:

- [`epd-waveshare`](https://crates.io/crates/epd-waveshare)
- [`ssd1680`](https://crates.io/crates/ssd1680)

## License

This crate is licenced under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
