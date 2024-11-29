use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_usurp_the_throne() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_156",
        name: "Usurp The Throne",
        boost: 3,
        card_icons: vec![CardIcon::Hazard],
        description: "",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_156.png",
        initial_threat: Count::PerPlayer(3),
    })
}
