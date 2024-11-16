use crate::systems::AppState;
use bevy::prelude::*;

#[derive(SubStates, Default, Hash, PartialEq, Eq, Debug, Clone)]
#[source(AppState = AppState::Collection)]
pub enum CollectionState {
    #[default]
    Menu,
    Hero,
    Basic,
    Aggression,
    Leadership,
    Protection,
    Justice,
    Pool,
    Villain,
    Modular,
}

pub struct CollectionStatePlugin;

impl Plugin for CollectionStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CollectionState>()
            .observe(change_app_state);
    }
}

#[derive(Event)]
pub struct CollectionStateChangeEvent(pub CollectionState);

fn change_app_state(
    trigger: Trigger<CollectionStateChangeEvent>,
    mut next_state: ResMut<NextState<CollectionState>>,
) {
    let CollectionStateChangeEvent(state) = trigger.event();
    next_state.set(state.clone());
}
