use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait},
};
pub fn get_tigra() -> Card {
    Card::Ally(AllyCard {
        id: "core_051",
        name: "Tigra",
        sub_name: "Greer Grant Nelson",
        aspect: CardAspect::Aggression,
        unique: true,
        cost: 3,
        res: vec![CardResource::Mental],
        initial_hit_points: 3,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        card_icons: vec![],
        thw: 1,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Response: After Tigra attacks and defeats a minion, heal 1 damage from her.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_051.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
    })
}
