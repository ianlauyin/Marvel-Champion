use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, UpgradeCard,
};

pub fn get_rocket_boots() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_039",
        name: "Rocket Boots",
        description: "You get +1 hit point. Hero Action: Exhaust Rocket Boots and spend a  resource -> gain the Aerial trait until the end of the phase.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_039.png",
        traits: vec![CardTrait::Armor, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: false,
        cost: 1,
        res: vec![CardResource::Mental],
        card_amount_max: 2,
    })
}
