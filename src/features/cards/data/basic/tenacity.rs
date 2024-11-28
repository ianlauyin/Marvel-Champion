use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, CardTrait, UpgradeCard},
};

pub fn get_tenacity() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_093",
        name: "Tenacity",
        aspect: CardAspect::Basic,
        unique: false,
        cost: 2,
        res: vec![CardResource::Energy],
        card_icons: vec![],
        traits: vec![CardTrait::Condition],
        keywords: vec![],
        description:
            "Hero Action: Spend a physical resource and discard this card → ready your hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_093.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 3,
    })
}
