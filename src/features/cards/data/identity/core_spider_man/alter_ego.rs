use crate::features::cards::{AlterEgoCard, Card, CardAspect, CardTrait, Identity::CoreSpiderMan};

pub fn get_alter_ego() -> Card {
    Card::AlterEgo(AlterEgoCard {
        id: "core_001b",
        name: "Peter Parker",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        flip_target_id: vec!["core_001a"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Genius],
        card_icons: vec![],
        rec: 3,
        description: "Resource: Generate a mental resource. (Limit once per round.)",
        abilities: vec![],
        hand_size: 6,
        card_image_path: "embedded://cards/identity/core_spider_man/core_001b.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168", "core_169"],
    })
}
