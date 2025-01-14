use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_assault_on_norad_2b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_138b",
        name: "Assault on NORAD - 2B",
        description: "Forced Response: After placing threat here during step one of the villain phase, each player must choose to either place 2 threat here or put the top card of their deck into play facedown, engaged with them as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_ultron/core_138b.png",
        next_stage_id: Some("core_139a"),
        target_threat: Count::PerPlayer(10),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
