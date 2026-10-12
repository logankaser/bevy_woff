## 0.3.0
Update to Bevy `0.20`.
No runtime changes.

## 0.2.0

- Updated Bevy dependencies to `0.19.0`.
- Migrated font loading from `Font::try_from_bytes` (removed in Bevy 0.19) to `Font::from_bytes`.
- Removed `WoffLoadError::Font` as font loading is now infallible at load-time (validation is deferred to rendering).
- Updated simple example and dev-dependencies to support Bevy 0.19 UI changes (`FontSize::Px` and `FontSource::Handle`).

## 0.1.0

- Initial release.
- Added `WoffPlugin` for loading `.woff` and `.woff2` files in Bevy.
- Added features `woff1` and `woff2` to toggle decompression algorithms.
