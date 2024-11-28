use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_titania() -> Card {
    Card::Minion(MinionCard {
        id: "core_162",
        name: "Titania",
        boost: 2,
        card_icons: vec![],
        description: "X is equal to Titania's remaining hit points.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_162.png",
        unique: true,
        initial_hit_points: 6,
        keywords: vec![],
        traits: vec![CardTrait::Brute, CardTrait::Elite],
        sch: 1,
        atk: 0,
    })
}
