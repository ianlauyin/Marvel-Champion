use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_armored_guard() -> Card {
    Card::Minion(MinionCard {
        id: "core_120",
        name: "Armored Guard",
        description: "Guard. (While this minion is engaged with you, you cannot attack the villain.) Toughness. (This character enters play with a tough status card.)",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_120.png",
        card_icons: vec![],
        boost: 3,
        traits: vec![CardTrait::Mercenary],
        unique: false,
        initial_hit_points: 3,
        keywords: vec![Keyword::Guard,Keyword::Toughness],
        sch: 0,
        atk: 1,
        boost_effect:None,
    })
}
