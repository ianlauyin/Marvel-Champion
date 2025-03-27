use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_181",
        name: "Madame Hydra",
        sub_name: None,
        unique: true,
        card_amount_max: 2,
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
            CardBoost::amount(2),
            CardTraits::single(CardTrait::Hydra),
            CardCharacter::minion(Count::Constant(6), 2, 2),
        ))
        .id()
}
