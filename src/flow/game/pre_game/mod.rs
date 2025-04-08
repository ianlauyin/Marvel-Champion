mod deck_menu;
mod hero_menu;
mod state;

use bevy::prelude::*;

pub struct PreGamePlugin;

impl Plugin for PreGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<state::PreGameState>()
            .add_plugins((deck_menu::DeckMenuPlugin, hero_menu::HeroMenuPlugin));
    }
}
