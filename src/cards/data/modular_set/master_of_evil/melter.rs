use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_melter() -> Card {
    Card::Minion(MinionCard {
        id: "core_132",
        name: "Melter",
        description:
            "The engaged player must defend against Melter's attacks with an ally they control, if able. Boost: Exhaust each ally you control.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_132.png",
        boost: 0,
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 5,
        keywords: vec![],
        sch: 1,
        atk: 3,
        boost_effect:None,
    })
}
