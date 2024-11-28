use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{Card, CardAspect, CardResource, CardTrait, EventCard},
};

pub fn get_emergency() -> Card {
    Card::Event(EventCard {
        id: "core_085",
        name: "Emergency",
        aspect: CardAspect::Basic,
        cost: 0,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Thwart],
        keywords: vec![],
        description: "Interrupt (thwart): When the villain schemes, reduce the amount of threat placed on the scheme by 1.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_085.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 3,
    })
}
