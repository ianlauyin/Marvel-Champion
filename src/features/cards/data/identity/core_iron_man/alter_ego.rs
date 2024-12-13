use crate::features::cards::{AlterEgoCard, Card, CardAspect, CardTrait, Identity::CoreIronMan};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_029b",
        name: "Tony Stark",
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        description: "Action: Look at the top 3 cards of your deck. Add 1 to your hand and discard the others. (Limit once per round.)",
        abilities: vec![],
        card_image_path:
            "embedded://cards/identity/core_iron_man/core_029b.png",
        traits: vec![CardTrait::Genius],
        flip_target_id: vec!["core_029a"],
        initial_hit_points: 9,
        keywords: vec![],
        card_icons: vec![],
        hand_size: 6,
        nemesis_id: "core_172",
        nemesis_side_scheme_id: "core_171",
        nemesis_card_id: vec!["core_173", "core_174"],
        rec: 3,
    })
}
