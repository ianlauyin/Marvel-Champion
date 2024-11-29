use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreSheHulk, UpgradeCard,
};

pub fn get_focused_rage() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_027",
        name: "Focused Rage",
        card_icons: vec![],
        description: "Hero Action: Exhaust Focused Rage and take 1 damage -> draw 1 card.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_she_hulk/core_027.png",
        keywords: vec![],
        traits: vec![CardTrait::Skill],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        unique: false,
        cost: 3,
        res: vec![CardResource::Energy],
        card_amount_max: 2,
    })
}
