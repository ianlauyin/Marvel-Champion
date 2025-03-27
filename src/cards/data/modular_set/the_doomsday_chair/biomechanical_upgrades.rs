use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_185",
        name: "Biomechanical Upgrades",
        sub_name: None,
        unique: true,
        card_amount_max: 3,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
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
            CardTraits::single(CardTrait::Tech),
            CardKeywords::single(CardKeyword::Surge),
        ))
        .id()
}
