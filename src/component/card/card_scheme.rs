use bevy::prelude::Component;

use super::shared::Count;

#[derive(Component)]
pub struct CardScheme {
    initial_threat: Count,
    target_threat: Option<Count>,
    increase_threat: Option<Count>,
}

impl CardScheme {
    pub fn new(initial_threat: Count) -> Self {
        Self {
            initial_threat,
            target_threat: None,
            increase_threat: None,
        }
    }

    pub fn main_scheme(
        initial_threat: Count,
        target_threat: Count,
        increase_threat: Count,
    ) -> Self {
        Self {
            initial_threat,
            target_threat: Some(target_threat),
            increase_threat: Some(increase_threat),
        }
    }
}
