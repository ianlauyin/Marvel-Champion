use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther, UpgradeCard,
};

pub fn get_vibranium_suit() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_049",
        name: "Vibranium Suit",
        description: "Special (attack): Move 1 damage from your hero to an enemy (2 damage instead if this is the final step of this sequence).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_049.png",
        traits: vec![CardTrait::BlackPanther, CardTrait::Armor],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Mental],
        card_amount_max: 1,
        unique: false,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
    })
}
