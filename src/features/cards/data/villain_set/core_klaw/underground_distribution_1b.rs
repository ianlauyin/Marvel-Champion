use crate::features::cards::{Card, Count, MainSchemeBCard};

pub fn get_underground_distribution_1b() -> Card {
    Card::MainSchemeB(MainSchemeBCard {
        id: "core_116b",
        name: "Underground Distribution - 1B",
        description: "When Revealed: Discard cards from the encounter deck until a minion is discarded. Put that minion into play engaged with the first player.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_116b.png",
        next_stage_id: Some("core_117a"),
        target_threat: Count::PerPlayer(6),
        increase_threat: Count::PerPlayer(1),
        initial_threat: Count::Constant(0),
        card_icons: vec![],
    })
}
