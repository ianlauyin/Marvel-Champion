use crate::features::cards::{Card, CardIcon, SideSchemeCard};

pub fn get_nemesis_side_scheme(player_number: u8) -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_166",
        name: "Highway Robbery",
        boost: 3,
        initial_threat: 3 * player_number,
        card_icons: vec![CardIcon::Acceleration],
        description:
            "When Revealed: Each player places a random card from their hand facedown here.
            When Defeated: Return each facedown card here to its owner's hand.",
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_166.png",
        card_back_image_path: "embedded://cards/card_backs_card/encounter_card_back.png",
    })
}
