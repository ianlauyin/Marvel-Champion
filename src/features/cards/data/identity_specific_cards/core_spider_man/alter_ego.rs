use crate::features::cards::{AlterEgoCard, Card, CardTrait};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_1b",
        name: "Peter Parker",
        flip_target_id: vec!["core_1a"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Genius],
        card_icons: vec![],
        rec: 3,
        description: "Scientist - Resource: Generate a  resource. (Limit once per round.)",
        abilities: vec![],
        hand_size: 6,
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_1b.png",
        card_back_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_1a.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168", "core_169"],
    })
}
