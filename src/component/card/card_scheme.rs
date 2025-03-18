use bevy::prelude::Component;

use super::shared::Count;

#[derive(Component)]
pub struct CardScheme {
    initial_threat: Count,
    target_threat: Option<Count>,
    increase_threat: Option<Count>,
}
