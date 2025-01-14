use crate::features::cards::{Card, MainSchemeACard};

pub fn get_the_break_in_1a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_097a",
        name: "The Break-In! - 1A",
        description: "Setup: Advance to stage 1B.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_097a.png",
        next_stage_id: Some("core_097b"),
    })
}
