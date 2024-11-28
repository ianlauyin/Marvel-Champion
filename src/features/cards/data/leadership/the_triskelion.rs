use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, SupportCard},
};
pub fn get_the_triskelion() -> Card {
    Card::Support(SupportCard {
        id: "core_073",
        name: "The Triskelion",
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Energy],
        traits: vec![],
        keywords: vec![],
        description:
            "Increase your ally limit by 1. (This allows you to control more than 3 allies.)",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_073.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
        unique: true,
        cost: 1,
        card_icons: vec![],
    })
}
