use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_madame_hydra() -> Card {
    Card::Minion(MinionCard {
        id: "core_181",
        name: "Madame Hydra",
        description:
            "Madame Hydra cannot take damage while the Legions of Hydra side scheme is in play. Forced Response: After Madame Hydra schemes or attacks, place 2 threat on the Legions of Hydra side scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/legions_of_hydra/core_181.png",
        boost: 2,
        traits: vec![CardTrait::Elite,CardTrait::Hydra],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 6,
        keywords: vec![],
        sch: 2,
        atk: 2,
        boost_effect:None,
    })
}
