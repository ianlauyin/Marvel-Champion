use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, EventCard},
};

pub fn get_first_aid() -> Card {
    Card::Event(EventCard {
        id: "core_086",
        name: "First Aid",
        aspect: CardAspect::Basic,
        cost: 1,
        res: vec![CardResource::Mental],
        traits: vec![],
        keywords: vec![],
        description: "Action: Heal 2 damage from any character.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core/core_086.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
    })
}
