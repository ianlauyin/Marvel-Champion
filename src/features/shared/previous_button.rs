use bevy::prelude::*;

use super::ButtonBuilder;

pub struct PreviousButtonPlugin;

impl Plugin for PreviousButtonPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct PreviousButtonBuilder;

const BUTTON_SIZE: (Val, Val) = (Val::Px(50.), Val::Px(50.));

impl PreviousButtonBuilder {
    pub fn spawn<'a>(&self, child_builder: &'a mut ChildBuilder) {
        let button = ButtonBuilder {
            text: "<",
            background_color: Color::NONE,
            size: BUTTON_SIZE,
            with_border: false,
        };
        button.spawn(child_builder);
    }
}
