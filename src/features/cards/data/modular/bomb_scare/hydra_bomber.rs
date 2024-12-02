use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_hydra_bomber() -> Card {
    Card::Minion(MinionCard {
        id: "core_110",
        name: "Hydra Bomber",
        description:
            "When Revealed: Choose to either take 2 damage or place 1 threat on the main scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/bomb_scare/core_110.png",
        boost: 1,
        traits: vec![CardTrait::Hydra],
        card_icons: vec![],
        unique: false,
        initial_hit_points: 2,
        keywords: vec![],
        sch: 1,
        atk: 1,
    })
}
