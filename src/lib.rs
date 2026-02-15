use bevy_app::{App, Plugin};
#[cfg(any(feature = "woff1", feature = "woff2"))]
use bevy_asset::{AssetApp, AssetLoader, LoadContext, io::Reader};
#[cfg(any(feature = "woff1", feature = "woff2"))]
use bevy_reflect::TypePath;
#[cfg(any(feature = "woff1", feature = "woff2"))]
use bevy_text::Font;

pub struct WoffPlugin;

impl Plugin for WoffPlugin {
    fn build(&self, #[allow(unused_variables)] app: &mut App) {
        #[cfg(feature = "woff1")]
        app.register_asset_loader(Woff1AssetLoader);
        #[cfg(feature = "woff2")]
        app.register_asset_loader(Woff2AssetLoader);
    }
}

#[cfg(any(feature = "woff1", feature = "woff2"))]
#[derive(Debug, thiserror::Error)]
enum WoffLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("woff decompression failed: {0}")]
    Decompress(wuff::WuffErr),
    #[error("invalid font data: {0}")]
    Font(Box<dyn std::error::Error + Send + Sync>),
}

#[cfg(any(feature = "woff1", feature = "woff2"))]
fn load_font(ttf_bytes: Vec<u8>) -> Result<Font, WoffLoadError> {
    Font::try_from_bytes(ttf_bytes).map_err(|e| WoffLoadError::Font(Box::new(e)))
}

#[cfg(feature = "woff1")]
#[derive(Default, TypePath)]
struct Woff1AssetLoader;

#[cfg(feature = "woff1")]
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
        let ttf_bytes = wuff::decompress_woff1(&woff_bytes).map_err(WoffLoadError::Decompress)?;
        load_font(ttf_bytes)
    }

    fn extensions(&self) -> &[&str] {
        &["woff"]
    }
}

#[cfg(feature = "woff2")]
#[derive(Default, TypePath)]
struct Woff2AssetLoader;

#[cfg(feature = "woff2")]
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
        let ttf_bytes = wuff::decompress_woff2(&woff2_bytes).map_err(WoffLoadError::Decompress)?;
        load_font(ttf_bytes)
    }

    fn extensions(&self) -> &[&str] {
        &["woff2"]
    }
}
