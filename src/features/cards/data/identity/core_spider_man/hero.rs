use crate::features::cards::{Card, CardAspect, CardTrait, HeroCard, Identity::CoreSpiderMan};
pub fn get_hero() -> Card {
    Card::Hero(HeroCard {
        id: "core_001a",
        name: "Spider-Man",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        flip_target_id: vec!["core_001b"],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        card_icons: vec![],
        thw: 1,
        atk: 2,
        def: 3,
        description: "Interrupt: When the villain initiates an attack against you, draw 1 card.",
        abilities: vec![],
        hand_size: 5,
        card_image_path: "embedded://cards/identity/core_spider_man/core_001a.png",
        nemesis_id: "core_167",
        nemesis_side_scheme_id: "core_166",
        nemesis_card_id: vec!["core_168", "core_169"],
    })
}
