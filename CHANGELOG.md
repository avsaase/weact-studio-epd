# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3](https://github.com/avsaase/weact-studio-epd/compare/v0.1.2...v0.1.3) - 2025-02-24

### Added

- *(color)* add conversion between `TriColor` and `Rgb888` (#24)

### Other

- Add comments to LUT

## [0.1.2](https://github.com/avsaase/weact-studio-epd/compare/v0.1.1...v0.1.2) - 2024-09-04

### Added
- Add sleep and wakeup support (with esp32c6 sample) ([#14](https://github.com/avsaase/weact-studio-epd/pull/14))
- Add additional color conversions for tinybmp support ([#16](https://github.com/avsaase/weact-studio-epd/pull/16))

### Fixed
- pass display by reference not by value for tricolor displays ([#15](https://github.com/avsaase/weact-studio-epd/pull/15))

### Other
- build esp32c6 examples ([#19](https://github.com/avsaase/weact-studio-epd/pull/19))
- rustfmt

## [0.1.1](https://github.com/avsaase/weact-studio-epd/releases/tag/v0.1.1) - 2024-08-01

### Added
- First release
