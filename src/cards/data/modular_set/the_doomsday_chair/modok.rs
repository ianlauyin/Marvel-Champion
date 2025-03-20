use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_184",
        name: "M.O.D.O.K.",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
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
            CardBoost::new(2),
            CardTraits::new(vec![CardTrait::Cyborg, CardTrait::Elite]),
            CardKeywords::single(CardKeyword::Retaliate(2)),
            CardCharacter::minion(Count::Constant(8), 2, 2),
        ))
        .id()
}
