use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_rhino_ii() -> Card {
    Card::Villain(VillainCard {
        id: "core_095",
        name: "Rhino (II)",
        initial_hit_points: Count::PerPlayer(15),
        keywords: vec![],
        traits: vec![CardTrait::Brute, CardTrait::Criminal],
        card_icons: vec![],
        sch: 1,
        atk: 3,
        description: "When Revealed: Search the encounter deck and discard pile for the Breakin' & Takin' side scheme and reveal it. Shuffle the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_095.png",
    })
}
