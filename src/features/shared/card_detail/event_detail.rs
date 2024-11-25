use bevy::prelude::*;

use crate::features::cards::EventCard;

pub fn spawn_event_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: EventCard<'static>,
) {
}
