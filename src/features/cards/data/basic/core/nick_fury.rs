use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait},
};
pub fn get_nick_fury() -> Card {
    Card::Ally(AllyCard {
        id: "core_084",
        name: "Nick Fury",
        sub_name: "Nick Fury",
        aspect: CardAspect::Basic,
        unique: true,
        cost: 4,
        res: vec![CardResource::Mental],
        initial_hit_points: 3,
        keywords: vec![],
        traits: vec![CardTrait::SHIELD, CardTrait::SPY],
        card_icons: vec![],
        thw: 2,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Forced Response: After Nick Fury enters play, choose one: remove 2 threat from a scheme, draw 3 cards, or deal 4 damage to an enemy. At the end of the round, if Nick Fury is still in play, discard him.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core/core_084.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 1,
    })
}
