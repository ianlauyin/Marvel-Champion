use bevy::app::{App, Plugin};

mod app_state;
mod asset_loader;
mod clean_up;
mod devtool;
mod setup;

pub struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SetupPlugin,
            app_state::AppStatePlugin,
            asset_loader::AssetLoaderSetupPlugin,
            devtool::DevtoolPlugin,
        ));
    }
}

pub use app_state::AppState;
pub use asset_loader::{AssetLoaderPlugin, LoadAsset};
pub use clean_up::clean_up;
