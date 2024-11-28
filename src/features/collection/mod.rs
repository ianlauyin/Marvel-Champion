use bevy::app::{App, Plugin};

mod aggression;
mod basic;
mod hero;
mod justice;
mod leadership;
mod menu;
mod pool;
mod protection;
mod state;

pub struct CollectionPlugin;

impl Plugin for CollectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            menu::CollectionMenuPlugin,
            state::CollectionStatePlugin,
            hero::CollectionHeroPlugin,
            basic::CollectionBasicPlugin,
            aggression::CollectionAggressionPlugin,
            justice::CollectionJusticePlugin,
            leadership::CollectionLeadershipPlugin,
            protection::CollectionProtectionPlugin,
            pool::CollectionPoolPlugin,
        ));
    }
}
