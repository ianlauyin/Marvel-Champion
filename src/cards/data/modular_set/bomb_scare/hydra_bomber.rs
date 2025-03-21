use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_110",
        name: "Hydra Bomber",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::BombScare).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Minion,
            CardBoost::amount(1),
            CardTraits::single(CardTrait::Hydra),
            CardCharacter::minion(Count::Constant(2), 1, 1),
        ))
        .id()
}
