mod components;
mod enemy_operation;
mod player_operation;
mod resources;
mod shared;

use bevy::app::{App, Plugin};
pub use enemy_operation::EnemyOperation;
pub use player_operation::PlayerOperation;
pub use resources::{FirstPlayer, PlayerNumber};

pub struct GameElementPlugin;

impl Plugin for GameElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            components::BelongsPlugin,
            components::GameUiPlugin,
            shared::SharedPlugin,
        ));
    }
}
