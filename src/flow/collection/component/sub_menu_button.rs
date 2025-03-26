use bevy::ecs::component::Component;

use crate::cards::SetTrait;

#[derive(Component)]
pub struct SubMenuButton(Box<dyn SetTrait>);

impl SubMenuButton {
    pub fn new(set: Box<dyn SetTrait>) -> Self {
        Self(set)
    }
}
