use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_163",
        name: "Genetically Enhanced",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreSheHulk).into(),
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
            CardTraits::single(CardTrait::Condition),
        ))
        .id()
}
