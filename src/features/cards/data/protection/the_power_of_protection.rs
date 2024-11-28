use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, ResourceCard},
};
pub fn get_the_power_of_protection() -> Card {
    Card::Resource(ResourceCard {
        id: "core_079",
        name: "Get Behind Me!",
        aspect: CardAspect::Protection,
        res: vec![CardResource::Wild],
        traits: vec![],
        description: "Max 2 per deck. Double the number of resources this card generates while paying for a Protection (green) card.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_079.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 2,
    })
}
