use crate::features::cards::{Card, CardTrait, Count, VillainCard};

pub fn get_rhino_i() -> Card {
    Card::Villain(VillainCard {
        id: "core_094",
        name: "Rhino (I)",
        initial_hit_points: Count::PerPlayer(14),
        keywords: vec![],
        traits: vec![CardTrait::Brute, CardTrait::Criminal],
        card_icons: vec![],
        sch: 1,
        atk: 2,
        description: "",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_094.png",
    })
}
