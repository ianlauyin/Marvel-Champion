mod card_list;
mod list;
mod state;

use bevy::app::{App, Plugin};

pub struct CollectionScenarioPlugin;

impl Plugin for CollectionScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            list::CollectionScenarioListPlugin,
            state::CollectionScenarioStatePlugin,
            card_list::CollectionScenarioCardListPlugin,
        ));
    }
}
