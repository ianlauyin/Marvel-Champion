use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, EventCard},
};
pub fn get_get_ready() -> Card {
    Card::Event(EventCard {
        id: "core_069",
        name: "Get Ready",
        cost: 0,
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Physical],
        traits: vec![],
        description: "Action: Ready an ally.",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_069.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
    })
}
