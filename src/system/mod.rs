mod app_state;
mod asset_loader;
mod setup;

pub const SYSTEM_PLUGINS: (
    setup::SetupPlugin,
    asset_loader::AssetLoaderPlugin,
    app_state::AppStatePlugin,
) = (
    setup::SetupPlugin,
    asset_loader::AssetLoaderPlugin,
    app_state::AppStatePlugin,
);

pub use app_state::{AppState, AppStateChangeEvent};
pub use asset_loader::{LoadAsset, StateLoading};
