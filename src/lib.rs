use bevy_app::{App, Plugin};
use bevy_asset::{io::Reader, AssetApp, AssetLoader, LoadContext};
use bevy_reflect::TypePath;
use bevy_text::Font;

pub struct WoffPlugin;

impl Plugin for WoffPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(Woff1AssetLoader);
        app.register_asset_loader(Woff2AssetLoader);
    }
}

#[derive(Debug, thiserror::Error)]
enum WoffLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("woff decompression failed: {0}")]
    Decompress(wuff::WuffErr),
    #[error("invalid font data: {0}")]
    Font(Box<dyn std::error::Error + Send + Sync>),
}

fn load_font(ttf_bytes: Vec<u8>) -> Result<Font, WoffLoadError> {
    Font::try_from_bytes(ttf_bytes).map_err(|e| WoffLoadError::Font(Box::new(e)))
}

#[derive(Default, TypePath)]
struct Woff1AssetLoader;

impl AssetLoader for Woff1AssetLoader {
    type Asset = Font;
    type Settings = ();
    type Error = WoffLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut woff_bytes = Vec::new();
        reader.read_to_end(&mut woff_bytes).await?;
        let ttf_bytes =
            wuff::decompress_woff1(&woff_bytes).map_err(WoffLoadError::Decompress)?;
        load_font(ttf_bytes)
    }

    fn extensions(&self) -> &[&str] {
        &["woff"]
    }
}

#[derive(Default, TypePath)]
struct Woff2AssetLoader;

impl AssetLoader for Woff2AssetLoader {
    type Asset = Font;
    type Settings = ();
    type Error = WoffLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut woff2_bytes = Vec::new();
        reader.read_to_end(&mut woff2_bytes).await?;
        let ttf_bytes =
            wuff::decompress_woff2(&woff2_bytes).map_err(WoffLoadError::Decompress)?;
        load_font(ttf_bytes)
    }

    fn extensions(&self) -> &[&str] {
        &["woff2"]
    }
}
