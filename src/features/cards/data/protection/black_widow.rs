use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait},
};
pub fn get_black_widow() -> Card {
    Card::Ally(AllyCard {
        id: "core_075",
        name: "Black Widow",
        sub_name: "Natasha Romanoff",
        unique: true,
        cost: 3,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Protection,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::SHIELD,CardTrait::Spy],
        initial_hit_points: 2,
        thw: 2,
        thw_con_dmg: 1,
        atk: 1,
        atk_con_dmg: 1,
        description: "Interrupt: When a card is revealed from the encounter deck, exhaust Black Widow and spend a Mental resource -> cancel the effects of that card and discard it. Then, reveal another card from the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_075.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
    })
}
