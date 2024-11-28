use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, UpgradeCard};
pub fn get_heroic_intuition() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_065",
        name: "Heroic Intuition",
        unique: false,
        cost: 2,
        card_icons: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Skill],
        keywords: vec![],
        description: "Play under any player's control. Max 1 per player. Your hero gets +1 THW.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_065.png",
        card_amount_max: 3,
    })
}
