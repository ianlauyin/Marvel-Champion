use bevy::prelude::Component;

#[derive(Component)]
pub enum IdentityCardType<'a> {
    AlterEgo { flip_target_id: Vec<&'a str> },
    Hero { flip_target_id: Vec<&'a str> },
}
