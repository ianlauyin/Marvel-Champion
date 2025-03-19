use bevy::ecs::world::World;

use crate::component::shared::Form;

pub struct Ability {
    limit: Option<Form>,
    func: fn(&mut World),
}

impl Ability {
    pub fn new(func: fn(&mut World)) -> Self {
        Self { limit: None, func }
    }

    pub fn hero(func: fn(&mut World)) -> Self {
        Self {
            limit: Some(Form::Hero),
            func,
        }
    }

    pub fn alter_ego(func: fn(&mut World)) -> Self {
        Self {
            limit: Some(Form::AlterEgo),
            func,
        }
    }
}
