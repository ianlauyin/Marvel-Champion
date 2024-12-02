use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_defence_network() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_125",
        name: "Defense Network",
        description: "When Revealed: Place an additional 1 perplaty threat here.",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_125.png",
        boost: 2,
        initial_threat: Count::Constant(2),
        card_icons: vec![CardIcon::Crisis],
    })
}
