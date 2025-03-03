use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreSheHulk, UpgradeCard,
};

pub fn get_superhuman_strength() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_028",
        name: "Superhuman Strength",
        card_icons: vec![],
        description: "She-Hulk gets +2 ATK. Forced Response: After She-Hulk attacks, discard Superhuman Strength -> stun the attacked enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_she_hulk/core_028.png",
        keywords: vec![],
        traits: vec![CardTrait::Superpower],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        unique: false,
        cost: 2,
        res: vec![CardResource::Mental],
        card_amount_max: 2,
    })
}
