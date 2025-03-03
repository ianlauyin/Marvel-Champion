use std::marker::PhantomData;

use super::CustomButton;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

/// Reminder: Add PreviousButtonPlugin::<State>::default() in state plugin
#[derive(Component)]
pub struct PreviousButton<S: FreelyMutableState>(pub S);

pub struct PreviousButtonPlugin<S>(PhantomData<S>);

impl<S> Default for PreviousButtonPlugin<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S: FreelyMutableState> Plugin for PreviousButtonPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_previous_interaction::<S>)
            .add_observer(handle_previous_button_spawn::<S>);
    }
}

fn handle_previous_button_spawn<S: FreelyMutableState>(
    on_add: Trigger<OnAdd, PreviousButton<S>>,
    mut commands: Commands,
) {
    commands.entity(on_add.entity()).insert(CustomButton {
        text: String::from("<"),
        color: Color::NONE,
        size: BUTTON_SIZE,
        with_border: false,
        ..default()
    });
}

fn handle_previous_interaction<S: FreelyMutableState>(
    previous_button_q: Query<(&Interaction, &PreviousButton<S>)>,
    mut next_state: ResMut<NextState<S>>,
) {
    if previous_button_q.is_empty() {
        return;
    }
    for (interaction, previous_button) in previous_button_q.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(previous_button.0.clone());
        }
    }
}
