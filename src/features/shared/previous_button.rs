use bevy::prelude::*;

pub struct PreviousButtonPlugin;

impl Plugin for PreviousButtonPlugin {
    fn build(&self, app: &mut App) {}
}

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

pub fn spawn_previous_button(child_builder: &mut ChildBuilder) {}
