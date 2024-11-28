use crate::features::cards::{Card, CardTrait, HeroCard};

pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_10a",
        name: "Captain Marvel",
        description:"Rechannel — Action: Spend a Energy resource and heal 1 damage from Captain Marvel -> draw 1 card. (Limit once per round.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_captain_marvel/core_010a.png",
        traits: vec![CardTrait::Avenger,CardTrait::Soldier],
        flip_target_id: vec!["core_1b"],
        initial_hit_points: 12,
        keywords: vec![],
        card_icons: vec![],
        thw: 2,
        atk: 2,
        def: 1,
        hand_size: 5,
        nemesis_id: "core_177",
        nemesis_side_scheme_id: "core_176",
        nemesis_card_id:vec!["core_178","core_179"],
    })
}
