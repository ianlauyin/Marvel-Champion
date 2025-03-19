use bevy::prelude::Component;

use crate::component::shared::Form;

#[derive(Component)]
pub struct CardFormLimit(Form);

impl CardFormLimit {
    pub fn hero() -> Self {
        Self(Form::Hero)
    }

    pub fn alter_ego() -> Self {
        Self(Form::AlterEgo)
    }
}
