use bevy::ecs::component::Component;

use crate::{cards::SetTrait, component::Card};

#[derive(Component)]
pub struct SubMenuButton(Box<dyn SetTrait>);

impl SubMenuButton {
    pub fn new(set: Box<dyn SetTrait>) -> Self {
        Self(set)
    }

    pub fn get_cards(&self) -> Vec<Card<'static>> {
        self.0.get_cards()
    }
}
