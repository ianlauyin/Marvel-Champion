use crate::features::cards::{AlterEgoCard, Card, CardTrait};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_040b",
        name: "T'Challa",
        description: "Setup: Search your deck for a Black Panther upgrade and add it to your hand. Shuffle your deck.",
        abilities: vec![],
        card_image_path:
            "embedded://cards/identity_specific_card/core_black_panther/core_040b.png",
        traits: vec![CardTrait::King, CardTrait::Wakanda],
        flip_target_id: vec!["core_040a"],
        initial_hit_points: 11,
        keywords: vec![],
        card_icons: vec![],
        hand_size: 6,
        nemesis_id: "core_157",
        nemesis_side_scheme_id: "core_156",
        nemesis_card_id:vec!["core_158","core_159"],
        rec: 4,
    })
}
