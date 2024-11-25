mod ally_detail;
mod alter_ego_detail;
mod attachment_detail;
mod card_detail;
mod environment_detail;
mod event_detail;
mod hero_detail;
mod main_scheme_detail;
mod minion_detail;
mod obligation_detail;
mod resource_detail;
mod side_scheme_detail;
mod support_detail;
mod treachery_detail;
mod upgrade_detail;
mod villain_detail;

use bevy::prelude::*;

pub struct CardDetailPlugin;

impl Plugin for CardDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(card_detail::CardDetailFramePlugin);
    }
}

pub use card_detail::spawn_card_detail;
