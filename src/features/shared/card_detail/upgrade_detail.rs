use bevy::prelude::*;

use crate::features::cards::UpgradeCard;

pub fn spawn_upgrade_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: UpgradeCard<'static>,
) {
}
