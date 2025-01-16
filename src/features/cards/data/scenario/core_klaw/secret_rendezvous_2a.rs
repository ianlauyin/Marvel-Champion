use crate::features::cards::{Card, MainSchemeACard};

pub fn get_secret_rendezvous_2a() -> Card {
    Card::MainSchemeA(MainSchemeACard {
        id: "core_117a",
        name: "Secret Rendezvous - 2A",
        description: "When Revealed: Discard cards from the encounter deck until a minion is discarded. Put that minion into play engaged with the first player. Advance to stage 2B",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_117a.png",
        card_back_image_path: "embedded://cards/scenario/core_klaw/core_117b.png",
        next_stage_id: Some("core_117b"),
    })
}
