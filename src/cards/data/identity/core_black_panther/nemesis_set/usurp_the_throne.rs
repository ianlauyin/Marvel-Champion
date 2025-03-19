use bevy::ecs::{system::Commands, world::World};

use crate::{
    cards::{Belong, IdentitySet},
    component::card::{CardBasic, CardBoost, CardIcon, CardIcons, CardScheme, Count},
};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_156",
        name: "Usurp The Throne",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        CardIcons::new(vec![CardIcon::Hazard]),
        CardBoost::new(3),
        CardScheme::new(Count::PerPlayer(3)),
    ));
}
