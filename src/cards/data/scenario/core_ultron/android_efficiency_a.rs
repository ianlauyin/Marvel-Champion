use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_144a",
        name: "Android Efficiency",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Scenario(Scenario::CoreUltron).into(),
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
            CardBoost::new(0),
            WhenRevealedAbilities::single(Ability::new(when_revealed_ability)),
            BoostAbilities::single(Ability::new(boost_ability)),
        ))
        .id()
}

fn when_revealed_ability(world: &mut World) {
    println!("when_revealed_ability");
}

fn boost_ability(world: &mut World) {
    println!("boost_ability");
}

// pub fn get_android_efficiency_a() -> Card {
//     Card::Treachery(TreacheryCard {
//         id: "core_144a",
//         name: "Android Efficiency",
//         traits: vec![],
//         description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Boost: Choose to either spend a Energy resource or put the top card of the deck into play facedown, engaged with you as a Drone minion.",
//         abilities: vec![],
//         card_image_path: "embedded://cards/scenario/core_ultron/core_144a.png",
//         boost: 0,
//         keywords: vec![],
//         boost_effect:None,
//     })
// }
