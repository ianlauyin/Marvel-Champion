use bevy::app::{App, Plugin};

mod app_state;
mod asset_loader;
mod setup;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SetupPlugin,
            asset_loader::AssetLoaderPlugin,
            app_state::AppStatePlugin,
        ));
    }
}

pub use app_state::{AppState, AppStateChangeEvent};
pub use asset_loader::{LoadAsset, StateLoading};
