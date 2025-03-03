use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, UpgradeCard};

pub fn get_combat_training() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_057",
        name: "Combat Training",
        unique: false,
        cost: 2,
        card_icons: vec![],
        aspect: CardAspect::Aggression,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Skill],
        keywords: vec![],
        description: "Play under any player's control. Max 1 per player. Your hero gets +1 ATK.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_057.png",
        card_amount_max: 3,
    })
}
