use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_169",
        name: "The Vulture's Plans",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
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
            CardBoost::amount(2),
        ))
        .id()
}
