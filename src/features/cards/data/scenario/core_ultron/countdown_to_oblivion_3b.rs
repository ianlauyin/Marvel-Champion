use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_countdown_to_oblivion_3b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_139b",
        name: "Countdown to Oblivion - 3B",
        description: "Threat cannot be removed from this scheme. If this stage is completed, the players lose the game",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_139b.png",
        next_stage_id: None,
        target_threat: Count::PerPlayer(5),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
