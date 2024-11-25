use bevy::prelude::*;

use crate::features::cards::ResourceCard;

pub fn spawn_resource_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: ResourceCard<'static>,
) {
}
