use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_whirlwind() -> Card {
    Card::Minion(MinionCard {
        id: "core_130",
        name: "Whirlwind",
        description:
            "Forced Interrupt: When Whirlwind attacks you, also resolve his attack against each other hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_130.png",
        boost: 0,
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 6,
        keywords: vec![],
        sch: 1,
        atk: 2,
        boost_effect:None,
    })
}
