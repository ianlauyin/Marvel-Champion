use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_153",
        name: "Concussion Blasters",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::ModularSet(ModularSet::UnderAttack).into(),
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
            EncounterCardType::Attachment,
            CardBoost::amount(1),
            CardTraits::single(CardTrait::Weapon),
            CardKeywords::single(CardKeyword::Retaliate(1)),
        ))
        .id()
}
