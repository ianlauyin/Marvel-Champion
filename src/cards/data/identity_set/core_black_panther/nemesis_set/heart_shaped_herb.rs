use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_158",
        name: "Heart-Shaped Herb",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
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
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
            BoostAbilities::single(Ability::new(boost_ability)),
            CardBoost::new(0),
        ))
        .id()
}

fn boost_ability(world: &mut World) {
    println!("boost_ability");
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}
