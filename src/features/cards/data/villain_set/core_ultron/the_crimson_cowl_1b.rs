use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_the_crimson_cowl_1b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_137b",
        name: "The Crimson Cowl - 1B",
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_137b.png",
        next_stage_id: Some("core_138a"),
        target_threat: Count::PerPlayer(3),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
