mod card_detail;

use bevy::prelude::*;

pub struct CardDetailPlugin;

impl Plugin for CardDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(card_detail::CardDetailFramePlugin);
    }
}

pub use card_detail::spawn_card_detail;
