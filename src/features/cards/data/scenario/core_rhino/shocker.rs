use crate::features::cards::{Card, CardTrait, MinionCard};

pub fn get_shocker() -> Card {
    Card::Minion(MinionCard {
        id: "core_103",
        name: "Shocker",
        description: "When Revealed: Deal 1 damage to each hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_103.png",
        card_icons: vec![],
        boost: 2,
        traits: vec![CardTrait::Criminal],
        unique: true,
        initial_hit_points: 3,
        keywords: vec![],
        sch: 1,
        atk: 2,
        boost_effect: None,
    })
}
