use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_159",
        name: "Ritual Combat",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
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
            CardBoost::new(2),
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
        ))
        .id()
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}
