use crate::features::cards::{Card, CardTrait, Count, Keyword, VillainCard};

pub fn get_rhino_iii() -> Card {
    Card::Villain(VillainCard {
        id: "core_096",
        name: "Rhino (III)",
        initial_hit_points: Count::PerPlayer(16),
        keywords: vec![Keyword::Toughness],
        traits: vec![CardTrait::Brute, CardTrait::Criminal],
        card_icons: vec![],
        sch: 1,
        atk: 3,
        description: "Toughness. (This character enter play with a tough status card.) When Revealed: Stun each hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_rhino/core_096.png",
    })
}
