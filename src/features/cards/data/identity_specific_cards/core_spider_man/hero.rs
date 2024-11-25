use crate::features::cards::{Card, CardTrait, HeroCard};
pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_1a",
        name: "Spider-Man",
        flip_target_id: vec!["core_1b"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        card_icons: vec![],
        thw: 1,
        atk: 2,
        def: 3,
        description: "Spider-Sense - Interrupt: When the villain initiates an attack against you, draw 1 card.",
        abilities: vec![],
        hand_size: 5,
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_1a.png",
        card_back_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_1b.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168","core_169"]
    })
}
