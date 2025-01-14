use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_crowd_control() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_108",
        name: "Crowd Control",
        description: "",
        abilities: vec![],
        traits: vec![],
        card_image_path: "embedded://cards/scenario/core_rhino/core_108.png",
        boost: 2,
        initial_threat: Count::PerPlayer(2),
        card_icons: vec![CardIcon::Crisis],
        boost_effect: None,
    })
}
