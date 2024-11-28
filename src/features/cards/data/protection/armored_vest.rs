use crate::features::cards::{Card, CardAspect, CardResource, CardTrait, UpgradeCard};
pub fn get_armored_vest() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_081",
        name: "Armored Vest",
        aspect: CardAspect::Protection,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Armor],
        keywords: vec![],
        description: "Play under any player's control. Max 1 per player. Your hero gets +1 DEF.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_081.png",
        card_amount_max: 3,
        unique: false,
        cost: 1,
        card_icons: vec![],
    })
}
