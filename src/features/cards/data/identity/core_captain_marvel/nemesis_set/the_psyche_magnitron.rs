use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_the_psyche_magnitron() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_176",
        name: "The Psyche-Magnitron",
        boost: 3,
        traits: vec![],
        card_icons: vec![CardIcon::Hazard],
        description: "When Revealed: Place an additional 1 per person threat here.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_176.png",
        initial_threat: Count::Constant(3),
    })
}
