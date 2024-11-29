use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, UpgradeCard,
};

pub fn get_mark_v_armor() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_036",
        name: "Mark V Armor",
        description: "You get +6 hit points.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_036.png",
        traits: vec![CardTrait::Armor, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 3,
        res: vec![CardResource::Mental],
        card_amount_max: 1,
    })
}
