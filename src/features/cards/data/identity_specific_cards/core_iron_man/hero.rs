use crate::features::cards::{Card, CardTrait, HeroCard};

pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_029a",
        name: "Iron Man",
        description:
            "You get +1 hand size for each Tech upgrade you control (to a maximum hand size of 7).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_029a.png",
        traits: vec![CardTrait::Avenger],
        flip_target_id: vec!["core_029b"],
        initial_hit_points: 9,
        keywords: vec![],
        card_icons: vec![],
        thw: 2,
        atk: 1,
        def: 1,
        hand_size: 1,
        nemesis_id: "core_172",
        nemesis_side_scheme_id: "core_171",
        nemesis_card_id: vec!["core_173", "core_174"],
    })
}
