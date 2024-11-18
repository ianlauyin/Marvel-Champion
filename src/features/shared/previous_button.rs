use super::ButtonBuilder;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

/// Reminder: Add handle_previous_interaction::<S> in system
pub struct PreviousButtonBuilder<S: FreelyMutableState>(pub S);

impl<S: FreelyMutableState + Clone> PreviousButtonBuilder<S> {
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
pub struct PreviousButton<S: FreelyMutableState>(S);

pub fn handle_previous_interaction<S: FreelyMutableState>(
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
