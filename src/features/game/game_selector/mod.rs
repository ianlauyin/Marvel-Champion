mod deck;
mod encounter;
mod identity;
mod state;

use bevy::prelude::*;

pub struct GameSelectorPlugin;

impl Plugin for GameSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            state::GameSelectorStatePlugin,
            identity::GameSelectorIdentityPlugin,
            deck::GameSelectorDeckPlugin,
            encounter::GameSelectorEncounterPlugin,
        ));
    }
}
