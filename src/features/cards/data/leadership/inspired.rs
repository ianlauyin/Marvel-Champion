use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, UpgradeCard},
};
pub fn get_inspired() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_074",
        name: "Inspired",
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Physical],
        traits: vec![],
        description: "Attach to an ally. Max 1 per ally. Attached ally gets +1 THW and +1 ATK.",
        abilities: vec![],
        keywords: vec![],
        card_image_path: "embedded://cards/leadership/core_074.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
        unique: false,
        cost: 1,
        card_icons: vec![],
    })
}
