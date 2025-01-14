use crate::features::cards::{Card, MainSchemeACard};

pub fn get_assault_on_norad_2a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_138a",
        name: "Assault on NORAD - 2A",
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Advance to stage 2B.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_138a.png",
        next_stage_id: Some("core_138b"),
    })
}
