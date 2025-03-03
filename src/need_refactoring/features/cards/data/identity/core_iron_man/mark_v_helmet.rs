use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, UpgradeCard,
};

pub fn get_mark_v_helmet() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_037",
        name: "Mark V Helmet",
        description: "Hero Action (thwart): Exhaust Mark V Helmet -> remove 1 threat from a scheme (from each scheme instead if you have the Aerial trait).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_037.png",
        traits: vec![CardTrait::Armor, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 1,
        res: vec![CardResource::Physical],
        card_amount_max: 1,
    })
}
