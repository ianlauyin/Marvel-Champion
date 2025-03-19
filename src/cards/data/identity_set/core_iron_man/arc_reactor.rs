use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, UpgradeCard,
};

pub fn get_arc_reactor() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_035",
        name: "Arc Reactor",
        description: "Hero Action: Exhaust Arc Reactor -> ready Iron Man.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_035.png",
        traits: vec![CardTrait::Item, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 2,
        res: vec![CardResource::Energy],
        card_amount_max: 1,
    })
}
