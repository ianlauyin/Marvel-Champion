use std::marker::PhantomData;

use super::CustomButton;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

/// Reminder: Add NextButtonPlugin::<State>::default() in state plugin
#[derive(Component)]
pub struct NextButton<S: FreelyMutableState>(pub S);

pub struct NextButtonPlugin<S>(PhantomData<S>);

impl<S> Default for NextButtonPlugin<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S: FreelyMutableState> Plugin for NextButtonPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_next_interaction::<S>)
            .add_observer(handle_next_button_spawn::<S>);
    }
}

fn handle_next_button_spawn<S: FreelyMutableState>(
    on_add: Trigger<OnAdd, NextButton<S>>,
    mut commands: Commands,
) {
    commands.entity(on_add.entity()).insert(CustomButton {
        text: String::from(">"),
        color: Color::NONE,
        size: BUTTON_SIZE,
        with_border: false,
        ..default()
    });
}

fn handle_next_interaction<S: FreelyMutableState>(
    next_button_q: Query<(&Interaction, &NextButton<S>)>,
    mut next_state: ResMut<NextState<S>>,
) {
    if next_button_q.is_empty() {
        return;
    }
    for (interaction, next_button) in next_button_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(next_button.0.clone());
        }
    }
}
