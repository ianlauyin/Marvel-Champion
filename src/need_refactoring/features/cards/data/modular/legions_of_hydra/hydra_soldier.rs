use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_hydra_soldier() -> Card {
    Card::Minion(MinionCard {
        id: "core_182",
        name: "Hydra Soldier",
        description:
            "Guard. (While this minion is engaged with you, you cannot attack the villain.) When Defeated: Deal the engaged player an encounter card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/legions_of_hydra/core_182.png",
        boost: 1,
        traits: vec![CardTrait::Hydra],
        card_icons: vec![],
        unique: false,
        initial_hit_points: 4,
        keywords: vec![Keyword::Guard],
        sch: 1,
        atk: 2,
        boost_effect:None,
    })
}
