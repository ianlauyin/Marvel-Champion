use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_178",
        name: "Kree Manipulator",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::IdentitySet(IdentitySet::CoreCaptainMarvel).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Treachery,
            CardKeywords::single(CardKeyword::Surge),
            CardBoost::amount(0),
        ))
        .id()
}
