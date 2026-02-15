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

Once the plugin is added, `.woff` and `.woff2` files can be loaded as `Font` assets through the `AssetServer` just like `.ttf` or `.otf` files.

## Compatibility

| bevy | bevy_woff |
|------|-----------|
| 0.18 | 0.1       |
