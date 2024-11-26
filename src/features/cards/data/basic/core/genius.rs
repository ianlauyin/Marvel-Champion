use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, ResourceCard},
};

pub fn get_genius() -> Card {
    Card::Resource(ResourceCard {
        id: "core_089",
        name: "Genius",
        aspect: CardAspect::Basic,
        res: vec![CardResource::Mental, CardResource::Mental],
        traits: vec![],
        description: "Max 1 per deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core/core_089.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 1,
    })
}
