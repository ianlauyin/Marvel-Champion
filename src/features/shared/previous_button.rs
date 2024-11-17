use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

use crate::systems::AppState;

use super::ButtonBuilder;

pub struct PreviousButtonPlugin;

impl Plugin for PreviousButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interaction::<AppState>);
    }
}

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

/// Reminder: Add handle_interaction in PreviousButtonPlugin when a new previous button is added.
pub struct PreviousButtonBuilder<T: FreelyMutableState>(pub T);

impl<T: FreelyMutableState + Clone> PreviousButtonBuilder<T> {
    pub fn spawn<'a>(&self, child_builder: &'a mut ChildBuilder) {
        let button = ButtonBuilder {
            text: "<",
            background_color: Color::NONE,
            size: BUTTON_SIZE,
            with_border: false,
        };
        button
            .spawn(child_builder)
            .insert(PreviousButton(self.0.clone()));
    }
}

#[derive(Component)]
struct PreviousButton<T: FreelyMutableState>(T);

fn handle_interaction<T: FreelyMutableState>(
    previous_button_q: Query<(&Interaction, &PreviousButton<T>)>,
    mut next_state: ResMut<NextState<T>>,
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
