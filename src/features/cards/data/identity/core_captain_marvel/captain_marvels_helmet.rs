use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreCaptainMarvel, UpgradeCard,
};
pub fn get_captain_marvels_helmet() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_016",
        name: "Captain Marvel's Helmet",
        unique: true,
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        cost: 2,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Armor, CardTrait::Tech],
        keywords: vec![],
        card_icons: vec![],
        description: "Captain Marvel gets +1 DEF (+2 DEF instead if you have the Aerial trait).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_016.png",
        card_amount_max: 1,
    })
}
