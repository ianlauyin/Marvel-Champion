use bevy::{
    ecs::system::{IntoObserverSystem, ObserverSystem},
    prelude::*,
};

#[derive(Component, Clone)]
pub enum CardState {
    InPlay,
    OutPlay,
}

impl CardState {
    pub fn toggle(&mut self) {
        *self = match *self {
            CardState::InPlay => CardState::OutPlay,
            CardState::OutPlay => CardState::InPlay,
        }
    }
}

pub fn change_card_state_on_added<C: Component>(
    target_state: CardState,
) -> impl ObserverSystem<OnAdd, C> {
    IntoObserverSystem::into_system(
        move |on_add: Trigger<OnAdd, C>, mut component_q: Query<&mut CardState, With<C>>| {
            let entity = on_add.entity();
            let Ok(mut card_state) = component_q.get_mut(entity) else {
                warn!("Cannot find CardState on entity: {:?}", entity);
                return;
            };
            *card_state = target_state.clone();
        },
    )
}
