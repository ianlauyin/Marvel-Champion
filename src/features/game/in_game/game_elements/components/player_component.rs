use bevy::prelude::Component;

/// From 1-4
#[derive(Component)]
pub struct PlayerComponent(usize);

impl PlayerComponent {
    pub fn new(player_tag: usize) -> Self {
        Self(player_tag)
    }
}
