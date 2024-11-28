use crate::features::cards::{Card, CardTrait, HeroCard};

pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_019a",
        name: "She-Hulk",
        card_icons: vec![],
        description: "Response: After you change to this form, deal 2 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_019a.png",
        flip_target_id: vec!["core_019b"],
        initial_hit_points: 15,
        keywords: vec![],
        traits: vec![CardTrait::Avenger, CardTrait::Gamma],
        thw: 1,
        atk: 3,
        def: 2,
        hand_size: 4,
        nemesis_id: "core_162",
        nemesis_side_scheme_id: "core_161",
        nemesis_card_id: vec!["core_163", "core_164"],
    })
}
