use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_bomb_scare() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_109",
        name: "Bomb Scare",
        description: "When Revealed: Place an additional 1 Per Person threat here.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/bomb_scare/core_109.png",
        boost: 2,
        traits: vec![],
        initial_threat: Count::Constant(2),
        card_icons: vec![CardIcon::Acceleration],
        boost_effect: None,
    })
}
