use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_killmonger() -> Card {
    Card::Minion(MinionCard {
        id: "core_157",
        name: "Killmonger",
        boost: 2,
        card_icons: vec![],
        description: "",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_157.png",
        unique: true,
        initial_hit_points: 5,
        keywords: vec![],
        traits: vec![CardTrait::Assassin, CardTrait::Elite, CardTrait::Mercenary],
        sch: 2,
        atk: 2,
    })
}
