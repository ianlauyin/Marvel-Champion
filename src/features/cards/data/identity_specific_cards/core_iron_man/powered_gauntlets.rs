use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, UpgradeCard,
};

pub fn get_powered_gauntlets() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_038",
        name: "Powered Gauntlets",
        description: "Hero Action (attack): Exhaust Powered Gauntlets -> deal 1 damage to an enemy (2 damage instead if you have the Aerial trait).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_038.png",
        traits: vec![CardTrait::Armor, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: false,
        cost: 2,
        res: vec![CardResource::Energy],
        card_amount_max: 2,
    })
}
