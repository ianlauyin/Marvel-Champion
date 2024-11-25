use bevy::prelude::*;

use crate::features::cards::SupportCard;

pub fn spawn_support_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: SupportCard<'static>,
) {
}
