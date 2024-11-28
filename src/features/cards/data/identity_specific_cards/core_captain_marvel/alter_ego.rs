use crate::features::cards::{AlterEgoCard, Card, CardTrait};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_010b",
        name: "Carol Danvers",
        description: "Action: Choose a player to draw 1 card. (Limit once per round.)",
        abilities: vec![],
        card_image_path:
            "embedded://cards/identity_specific_card/core_captain_marvel/core_010b.png",
        traits: vec![CardTrait::SHIELD, CardTrait::Soldier],
        flip_target_id: vec!["core_010a"],
        initial_hit_points: 12,
        keywords: vec![],
        card_icons: vec![],
        hand_size: 6,
        nemesis_id: "core_177",
        nemesis_side_scheme_id: "core_176",
        nemesis_card_id: vec!["core_178", "core_179"],
        rec: 4,
    })
}
