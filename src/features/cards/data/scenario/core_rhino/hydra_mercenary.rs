use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_hydra_mercenary() -> Card {
    Card::Minion(MinionCard {
        id: "core_101",
        name: "Hydra Mercenary",
        description:
            "Guard. (While this minion is engaged with you, you cannot attack the villain.)",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_rhino/core_101.png",
        card_icons: vec![],
        boost: 1,
        traits: vec![CardTrait::Hydra],
        unique: false,
        initial_hit_points: 3,
        keywords: vec![Keyword::Guard],
        sch: 0,
        atk: 1,
        boost_effect: None,
    })
}
