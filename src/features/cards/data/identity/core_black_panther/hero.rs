use crate::features::cards::{Card, CardTrait, HeroCard, Keyword};

pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_040a",
        name: "Black Panther",
        description:"Retaliate 1. (After this character is attacked, deal 1 damage to the attacking character.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_040a.png",
        traits: vec![CardTrait::Avenger,CardTrait::Wakanda],
        flip_target_id: vec!["core_040b"],
        initial_hit_points: 11,
        keywords: vec![Keyword::Retaliate(1)],
        card_icons: vec![],
        thw: 2,
        atk: 2,
        def: 2,
        hand_size: 5,
        nemesis_id: "core_157",
        nemesis_side_scheme_id: "core_156",
        nemesis_card_id:vec!["core_158","core_159"],
    })
}
