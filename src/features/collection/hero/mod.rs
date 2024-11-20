mod card_list;
mod list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionHeroPlugin;

impl Plugin for CollectionHeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            list::CollectionHeroListPlugin,
            state::CollectionHeroStatePlugin,
            card_list::CollectionHeroCardListPlugin,
        ));
    }
}
