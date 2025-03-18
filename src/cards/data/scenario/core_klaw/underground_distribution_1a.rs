use crate::features::cards::{Card, MainSchemeACard};

pub fn get_underground_distribution_1a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_116a",
        name: "Underground Distribution - 1A",
        description: "Setup: Search the encounter deck for the Defense Network side scheme and reveal it. Shuffle the encounter deck. Advance to stage 1B.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_116a.png",
        card_back_image_path: "embedded://cards/scenario/core_klaw/core_116b.png",
        next_stage_id: Some("core_116b"),
    })
}
