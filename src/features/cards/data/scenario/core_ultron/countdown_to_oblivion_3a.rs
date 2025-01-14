use crate::features::cards::{Card, MainSchemeACard};

pub fn get_countdown_to_oblivion_3a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_139a",
        name: "Countdown to Oblivion - 3A",
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Advance to stage 3B.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_139a.png",
        next_stage_id: Some("core_139b"),
    })
}
