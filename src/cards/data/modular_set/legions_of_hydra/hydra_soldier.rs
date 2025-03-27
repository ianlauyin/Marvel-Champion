use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_182",
        name: "Hydra Soldier",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::ModularSet(ModularSet::LegionsOfHydra).into(),
        is_vertical: true,
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
            CardKeywords::single(CardKeyword::Guard),
            CardTraits::single(CardTrait::Hydra),
            CardCharacter::minion(Count::Constant(4), 1, 2),
        ))
        .id()
}
