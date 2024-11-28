use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, ResourceCard},
};

pub fn get_energy() -> Card {
    Card::Resource(ResourceCard {
        id: "core_088",
        name: "Energy",
        aspect: CardAspect::Basic,
        res: vec![CardResource::Energy, CardResource::Energy],
        traits: vec![],
        description: "Max 1 per deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_088.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
    })
}
