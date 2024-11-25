use bevy::prelude::*;

use crate::features::cards::AttachmentCard;

pub fn spawn_attachment_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: AttachmentCard<'static>,
) {
}
