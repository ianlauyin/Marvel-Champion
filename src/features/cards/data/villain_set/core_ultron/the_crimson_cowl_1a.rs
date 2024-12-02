use crate::features::cards::{Card, MainSchemeACard};

pub fn get_the_crimson_cowl_1a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_137a",
        name: "The Crimson Cowl - 1A",
        description: "Setup: Put the Ultron Drones environment into play. Shuffle the encounter deck. Advanced to stage 1B.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_137a.png",
        next_stage_id: Some("core_137b"),
    })
}
