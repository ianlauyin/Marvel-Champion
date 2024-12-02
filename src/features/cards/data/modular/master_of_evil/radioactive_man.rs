use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_radioactive_man() -> Card {
    Card::Minion(MinionCard {
        id: "core_129",
        name: "Radioactive Man",
        description:
            "Forced Response: After Radioactive Man attacks you, discard 1 card at random from your hand. Boost: Discard 1 card at random from your hand.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_129.png",
        boost: 0,
        traits: vec![CardTrait::Elite,CardTrait::MasterOfEvil],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 7,
        keywords: vec![],
        sch: 1,
        atk: 1,
    })
}
