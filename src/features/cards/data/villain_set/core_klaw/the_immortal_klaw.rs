use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_the_immortal_klaw() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_127",
        name: "The \"Immortal\" Klaw",
        description:
            "Klaw gets +10 hit points. (When this scheme is defeated, Klaw loses those hit points.)",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_127.png",
        boost: 0,
        initial_threat: Count::PerPlayer(3),
        card_icons: vec![CardIcon::Acceleration],
    })
}
