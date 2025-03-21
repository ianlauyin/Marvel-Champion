use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_168",
        name: "Sweeping Swoop",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::IdentitySet(IdentitySet::CoreSpiderMan).into(),
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
            CardCost::constant(0),
        ))
        .id()
}
