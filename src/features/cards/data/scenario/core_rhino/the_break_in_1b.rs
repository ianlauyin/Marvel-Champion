use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_the_break_in_1b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_097b",
        name: "The Break-In! - 1B",
        description: "If this stage is completed, the players lose the game.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_097b.png",
        next_stage_id: None,
        target_threat: Count::PerPlayer(7),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
