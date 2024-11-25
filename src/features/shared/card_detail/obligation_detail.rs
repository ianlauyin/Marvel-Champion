use bevy::prelude::*;

use crate::features::cards::ObligationCard;

pub fn spawn_obligation_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: ObligationCard<'static>,
) {
}
