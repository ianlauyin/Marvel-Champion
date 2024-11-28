use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_yogg_rogg() -> Card {
    Card::Minion(MinionCard {
        id: "core_177",
        name: "Yon-Rogg",
        boost: 2,
        card_icons: vec![],
        description:
            "Forced Response: After Yon-Rogg attacks, place 1 threat on The Psyche-Magnitron.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_captain_marvel/core_177.png",
        unique: true,
        initial_hit_points: 5,
        keywords: vec![],
        traits: vec![CardTrait::Elite, CardTrait::Kree],
        sch: 2,
        atk: 3,
    })
}
