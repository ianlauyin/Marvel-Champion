use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreCaptainMarvel, UpgradeCard,
};
pub fn get_cosmic_flight() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_017",
        name: "Cosmic Flight",
        unique: false,
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        cost: 2,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Superpower],
        keywords: vec![],
        card_icons: vec![],
        description: "Captain Marvel gains the Aerial trait. Hero Interrupt (defense): When Captain Marvel would take damage, discard Cosmic Flight -> prevent 3 of that damage.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_017.png",
        card_amount_max: 2,
    })
}
