use crate::features::cards::{AlterEgoCard, Card, CardTrait};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_019b",
        name: "Jennifer Walters",
        card_icons: vec![],
        description: "Interrupt: When threat would be placed on a scheme, prevent 1 of that threat. (Limit once per round.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_019b.png",
        flip_target_id: vec!["core_019a"],
        initial_hit_points: 15,
        keywords: vec![],
        traits: vec![CardTrait::Attorney, CardTrait::Gamma],
        hand_size: 6,
        nemesis_id: "core_162",
        nemesis_side_scheme_id: "core_161",
        nemesis_card_id: vec!["core_163", "core_164"],
        rec: 5,
    })
}
