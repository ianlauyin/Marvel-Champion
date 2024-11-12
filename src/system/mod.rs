mod asset_loader;
mod setup;
mod state;

pub const SYSTEM_PLUGINS: (
    setup::SetupPlugin,
    asset_loader::AssetLoaderPlugin,
    state::app_state::AppStatePlugin,
) = (
    setup::SetupPlugin,
    asset_loader::AssetLoaderPlugin,
    state::app_state::AppStatePlugin,
);

pub use asset_loader::{LoadAsset, StateLoading};
pub use state::app_state::{AppState, AppStateChangeEvent};
