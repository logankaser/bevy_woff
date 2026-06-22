# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-06-22

### Changed

- Updated Bevy dependencies to `0.19.0`.
- Migrated font loading from `Font::try_from_bytes` (removed in Bevy 0.19) to `Font::from_bytes`.
- Removed `WoffLoadError::Font` as font loading is now infallible at load-time (validation is deferred to rendering).
- Updated simple example and dev-dependencies to support Bevy 0.19 UI changes (`FontSize::Px` and `FontSource::Handle`).

## [0.1.0] - 2026-02-15

### Added

- Initial release.
- Added `WoffPlugin` for loading `.woff` and `.woff2` files in Bevy.
- Added features `woff1` and `woff2` to toggle decompression algorithms.
