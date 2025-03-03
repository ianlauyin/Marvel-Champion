use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_nemesis() -> Card {
    Card::Minion(MinionCard {
        id: "core_167",
        name: "Vulture",
        unique: true,
        initial_hit_points: 4,
        keywords: vec![Keyword::Quickstrike],
        traits: vec![CardTrait::Criminal],
        card_icons: vec![],
        sch: 1,
        atk: 3,
        boost: 2,
        description: "Quickstrike. (After this minion engages your hero, it attacks.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_spider_man/core_167.png",
        boost_effect: None,
    })
}
