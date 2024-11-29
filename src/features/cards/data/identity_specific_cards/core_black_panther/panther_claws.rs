use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther, UpgradeCard,
};

pub fn get_panther_claws() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_047",
        name: "Panther Claws",
        description: "Special (attack): Deal 2 damage to an enemy (4 damage instead if this is the final step of this sequence).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_047.png",
        traits: vec![CardTrait::BlackPanther, CardTrait::Weapon],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Energy],
        card_amount_max: 1,
        unique: false,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
    })
}
