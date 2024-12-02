use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_tiger_shark() -> Card {
    Card::Minion(MinionCard {
        id: "core_131",
        name: "Tiger Shark",
        description:
            "Forced Response: After Tiger Shark attacks, give him a tough status card. Boost: Give the villain a tough status card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_131.png",
        boost: 0,
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 6,
        keywords: vec![],
        sch: 1,
        atk: 3,
        boost_effect:None,
    })
}
