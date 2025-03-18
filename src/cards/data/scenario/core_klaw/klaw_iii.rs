use crate::features::cards::{Card, CardTrait, Count, Keyword, VillainCard};

pub fn get_klaw_iii() -> Card {
    Card::Villain(VillainCard {
        id: "core_115",
        name: "Klaw (III)",
        initial_hit_points: Count::PerPlayer(22),
        keywords: vec![Keyword::Toughness],
        traits: vec![CardTrait::MasterOfEvil],
        card_icons: vec![],
        sch: 3,
        atk: 2,
        description: "Toughness. (This character enters play with a tough status card.) Forced Interrupt: When Klaw attacks, give him 1 additional boost card for this activation.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_115.png",
    })
}
