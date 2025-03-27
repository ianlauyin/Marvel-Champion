use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_109",
        name: "Bomb Scare",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::BombScare).into(),
        is_vertical: false,
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::SideScheme,
            CardBoost::amount(2),
            CardIcons::acceleration(),
            CardScheme::new(Count::Constant(2)),
        ))
        .id()
}
