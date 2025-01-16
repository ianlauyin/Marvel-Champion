use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_secret_rendezvous_2b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_117b",
        name: "Secret Rendezvous - 2B",
        description: "If this stage is completed, the players lose the game.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_117b.png",
        card_back_image_path: "embedded://cards/scenario/core_klaw/core_117a.png",
        next_stage_id: None,
        target_threat: Count::PerPlayer(8),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
