use crate::features::cards::{builders::*, CardTrait};

pub fn get_hero() -> HeroCard {
    HeroCard {
        id: String::from("core_1a"),
        name: String::from("Spider-Man"),
        flip_target_id: vec![String::from("core_1b")],
        initial_hit_points: 10,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        card_icons: vec![],
        thw: 1,
        atk: 2,
        def: 3,
        description: String::from("Spider-Sense — Interrupt: When the villain initiates an attack against you, draw 1 card."),
        search_keywords: vec![],
        hand_size: 5,
        nemensis_id: String::from("core_167"),
        image_path: String::from("embedded://cards/identity_specific_cards/core_spider_man/core_1a.png"),
    }
}
