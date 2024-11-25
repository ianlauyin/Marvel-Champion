use bevy::prelude::*;

use crate::features::cards::EnvironmentCard;

pub fn spawn_environment_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: EnvironmentCard<'static>,
) {
}
