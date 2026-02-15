use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_woff::WoffPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            WoffPlugin,
        ))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
            ..default()
        },
    ));

    let woff2_font = asset_server.load::<Font>("fonts/PublicSans-Regular.woff2");
    let woff_font = asset_server.load::<Font>("fonts/PublicSans-Italic.woff");
    let ttf_font = asset_server.load::<Font>("fonts/PublicSans-ExtraBoldItalic.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                Text::new("Hello from .woff2! (Public Sans Regular)"),
                TextFont {
                    font: woff2_font,
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ),
            (
                Text::new("Hello from .woff! (Public Sans Italic)"),
                TextFont {
                    font: woff_font,
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ),
            (
                Text::new("Hello from .ttf! (Public Sans ExtraBold Italic)"),
                TextFont {
                    font: ttf_font,
                    font_size: 48.0,
                    weight: FontWeight::EXTRA_BOLD,
                    ..default()
                },
                TextColor(Color::BLACK),
            ),
        ],
    ));
}
