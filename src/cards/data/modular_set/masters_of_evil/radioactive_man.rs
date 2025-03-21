use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_129",
        name: "Radioactive Man",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::MastersOfEvil).into(),
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
            CardBoost::amount(0),
            CardTraits::new(vec![CardTrait::Elite, CardTrait::MastersOfEvil]),
            CardCharacter::minion(Count::Constant(7), 1, 1),
        ))
        .id()
}
