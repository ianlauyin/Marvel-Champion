use bevy::prelude::*;

use super::state::CollectionState;
pub struct HeroMenuPlugin;

impl Plugin for HeroMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Hero), spawn_hero_menu);
    }
}

fn spawn_hero_menu(mut commands: Commands) {}
