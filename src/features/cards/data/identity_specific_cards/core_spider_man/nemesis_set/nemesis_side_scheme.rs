use bevy::{
    ecs::{schedule::SystemConfigs, system::RunSystemOnce},
    prelude::{Commands, IntoSystem, IntoSystemConfigs, System, World},
};

use crate::{
    constants::ENCOUNTER_CARD_BACK_PATH,
    features::cards::{Card, CardAbility, CardIcon, SideSchemeCard},
};

pub fn get_nemesis_side_scheme(player_number: u8) -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_166",
        name: "Highway Robbery",
        boost: 3,
        initial_threat: 3 * player_number,
        card_icons: vec![CardIcon::Acceleration],
        description:
            "When Revealed: Each player places a random card from their hand facedown here.
            When Defeated: Return each facedown card here to its owner's hand.",
        abilities: vec![
            CardAbility::WhenRevealed(when_revealed()),
            CardAbility::WhenDefeated(when_defeated()),
        ],
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_166.png",
        card_back_image_path: ENCOUNTER_CARD_BACK_PATH,
    })
}

fn when_revealed() -> SystemConfigs {
    IntoSystem::into_system(|| println!("Hi")).into_configs()
}

fn when_defeated() -> SystemConfigs {
    IntoSystem::into_system(|| println!("Bye")).into_configs()
}
