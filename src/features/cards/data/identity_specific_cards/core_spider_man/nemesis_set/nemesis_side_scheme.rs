use bevy::{ecs::system::RunSystemOnce, prelude::World};

use crate::{
    constants::ENCOUNTER_CARD_BACK_PATH,
    features::cards::{Card, CardAbility, CardIcon, Count, SideSchemeCard},
};

pub fn get_nemesis_side_scheme() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_166",
        name: "Highway Robbery",
        boost: 3,
        initial_threat: Count::PerPlayer(3),
        card_icons: vec![CardIcon::Acceleration],
        description:
            "When Revealed: Each player places a random card from their hand facedown here.
            \nWhen Defeated: Return each facedown card here to its owner's hand.",

        abilities: vec![
            CardAbility::WhenRevealed(when_revealed),
            CardAbility::WhenDefeated(when_defeated),
        ],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_166.png",
        card_back_image_path: ENCOUNTER_CARD_BACK_PATH,
    })
}

fn when_revealed(world: &mut World) {
    world.run_system_once(|| println!("hi"));
}

fn when_defeated(world: &mut World) {
    world.run_system_once(|| println!("Bye"));
}
