use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther, UpgradeCard,
};

pub fn get_tactical_genius() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_048",
        name: "Tactical Genius",
        description: "Special (thwart): Remove 1 threat from a scheme (2 threat instead if this is the final step of this sequence).",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_048.png",
        traits: vec![CardTrait::BlackPanther, CardTrait::Skill],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        res: vec![CardResource::Physical],
        card_amount_max: 1,
        unique: false,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
    })
}
