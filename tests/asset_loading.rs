use bevy_app::{App, TaskPoolPlugin};
use bevy_asset::{AssetApp, AssetPlugin, AssetServer, Assets};
use bevy_text::{Font, FontLoader};
use bevy_woff::WoffPlugin;

fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        TaskPoolPlugin::default(),
        AssetPlugin {
            file_path: "assets".to_string(),
            ..Default::default()
        },
    ))
    .init_asset::<Font>();
    app
}

fn poll_asset(app: &mut App, id: bevy_asset::AssetId<Font>) -> Font {
    for _ in 0..100 {
        app.update();
        if let Some(font) = app.world().resource::<Assets<Font>>().get(id) {
            return font.clone();
        }
    }
    panic!("Font failed to load within 100 updates");
}

#[test]
fn load_ttf_font() {
    let mut app = test_app();
    app.register_asset_loader(FontLoader);

    let asset_server = app.world().resource::<AssetServer>().clone();
    let handle = asset_server.load::<Font>("fonts/PublicSans-ExtraBoldItalic.ttf");

    let font = poll_asset(&mut app, handle.id());
    assert!(!font.data.is_empty());
}

#[test]
fn load_woff_font() {
    let mut app = test_app();
    app.add_plugins(WoffPlugin);

    let asset_server = app.world().resource::<AssetServer>().clone();
    let handle = asset_server.load::<Font>("fonts/PublicSans-Italic.woff");

    let font = poll_asset(&mut app, handle.id());
    assert!(!font.data.is_empty());
}

#[test]
fn load_woff2_font() {
    let mut app = test_app();
    app.add_plugins(WoffPlugin);

    let asset_server = app.world().resource::<AssetServer>().clone();
    let handle = asset_server.load::<Font>("fonts/PublicSans-Regular.woff2");

    let font = poll_asset(&mut app, handle.id());
    assert!(!font.data.is_empty());
}
