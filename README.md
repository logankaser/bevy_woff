# bevy_woff

[![crates.io](https://img.shields.io/crates/v/bevy_woff)](https://crates.io/crates/bevy_woff)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevy.org/learn/quick-start/plugin-development/#main-branch-tracking)

A [Bevy](https://bevyengine.org/) plugin that adds asset loading support for `.woff` and `.woff2` web font files.

## Usage

```rust
use bevy::prelude::*;
use bevy_woff::WoffPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WoffPlugin))
        .run()
}
```

Once the plugin is added, `.woff2` files can be loaded as `Font` assets through the `AssetServer` just like `.ttf` or `.otf` files.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `woff2` | Yes | Adds `.woff2` file support (pulls in brotli decompression) |
| `woff1` | No | Adds `.woff` file support (pulls in zlib decompression) |

To enable both formats:

```toml
bevy_woff = { version = "0.1", features = ["woff1"] }
```

To use only `.woff` without `.woff2`:

```toml
bevy_woff = { version = "0.1", default-features = false, features = ["woff1"] }
```

## Compatibility

| bevy | bevy_woff |
|------|-----------|
| 0.18 | 0.1       |
