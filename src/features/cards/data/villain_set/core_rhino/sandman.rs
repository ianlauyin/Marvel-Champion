use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_sandman() -> Card {
    Card::Minion(MinionCard {
        id: "core_102",
        name: "Sandman",
        description: "Toughness. (This character enters play with a tough status card.)",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_102.png",
        card_icons: vec![],
        boost: 2,
        traits: vec![CardTrait::Criminal, CardTrait::Elite],
        unique: true,
        initial_hit_points: 4,
        keywords: vec![Keyword::Toughness],
        sch: 2,
        atk: 3,
    })
}
