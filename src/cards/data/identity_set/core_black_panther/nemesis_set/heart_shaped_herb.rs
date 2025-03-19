use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_158",
        name: "Heart-Shaped Herb",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        EncounterCardType::Treachery,
        CardKeywords::single(CardKeyword::Surge),
        InstantAbilities::single(instant_ability),
        CardBoost::new(0),
    ));
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
