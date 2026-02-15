# bevy_woff

[![crates.io](https://img.shields.io/crates/v/bevy_woff)](https://crates.io/crates/bevy_woff)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevy.org/learn/quick-start/plugin-development/#main-branch-tracking)

A [Bevy](https://bevyengine.org/) plugin that adds asset loading support for `.woff` and `.woff2` web font files.

As woff and woff2 are basically just the normal otf/ttf font format with a custom precompression
pass and some extra metadata. All this crate does is use `wuff`, a pure Rust woff parser,
to decompress the font and discard the extra metadata. Then the font data is just passed to the normal
bevy font loader.

This crate explicitly supports the web platform as well; any breakage on the web would
undermine the utility of being able to load web fonts from bevy in the first place.

## Usage

```rust
use bevy::prelude::*;
use bevy_woff::WoffPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WoffPlugin))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load::<Font>("my_font.woff2");
    // ...
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

## Future

This crate is essentially feature complete for my needs, although I'll continue to track
bevy release versions and wuff updates if any.

I may investigate using web-sys to do the decompression in JS using the browser API, when compiled for wasm32.
That is unlikely to be faster because of the double copy in and out of WASM memory.
It might save a bit of code size though.
I think it's fairly unlikely to be worth it, as the code is already very small, but it seems interesting to try.
Such a thing would be a non-default feature if I do implement it.
