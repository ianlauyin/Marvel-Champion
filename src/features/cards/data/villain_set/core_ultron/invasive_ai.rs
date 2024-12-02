use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_invasive_ai() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_149",
        name: "Invasive AI",
        description: "When Revealed: Each player discards the top 3 cards of their deck.",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_149.png",
        boost: 2,
        initial_threat: Count::PerPlayer(3),
        card_icons: vec![CardIcon::Hazard],
    })
}
