use bevy::prelude::*;

use super::state::CollectionState;

pub struct CollectionMenuPlugin;

impl Plugin for CollectionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Menu), || {
            print!("Enter Collection Menu")
        });
    }
}
