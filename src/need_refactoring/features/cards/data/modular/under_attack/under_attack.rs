use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_under_attack() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_151",
        name: "Under Attack",
        description:
            "When Revealed: Each player chooses to either place 2 threat here or deal 3 damage to their hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/under_attack/core_151.png",
        boost: 3,
        traits: vec![],
        card_icons: vec![CardIcon::Crisis],
        initial_threat: Count::Constant(3),
        boost_effect:None,
    })
}
