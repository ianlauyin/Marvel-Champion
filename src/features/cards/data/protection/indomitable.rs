use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, UpgradeCard},
};
pub fn get_indomitable() -> Card {
    Card::Upgrade(UpgradeCard {
        id: "core_082",
        name: "Indomitable",
        aspect: CardAspect::Protection,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Condition],
        keywords: vec![],
        description: "Response: After your hero defends, discard indomitable -> ready your hero.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_082.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
        unique: false,
        cost: 1,
        card_icons: vec![],
    })
}
