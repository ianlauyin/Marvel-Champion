use bevy::prelude::*;

use crate::features::cards::HeroCard;

pub fn spawn_hero_detail(
    children_builder: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    card: HeroCard,
) {
    children_builder.spawn({
        NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            ..default()
        }
    });
}
