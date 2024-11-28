use crate::{
    constants::PLAYER_CARD_BACK_ASSET,
    features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait},
};
pub fn get_hulk() -> Card {
    Card::Ally(AllyCard {
        id: "core_050",
        name: "Hulk",
        sub_name: "Bruce Banner",
        aspect: CardAspect::Aggression,
        unique: true,
        cost: 2,
        res: vec![CardResource::Energy],
        initial_hit_points: 5,
        keywords: vec![],
        traits: vec![CardTrait::Avenger, CardTrait::Gamma],
        card_icons: vec![],
        thw: 0,
        thw_con_dmg: 0,
        atk: 3,
        atk_con_dmg: 1,
        description: "Forced Response: After Hulk attacks, discard the top card of your deck. If that card's printed resource has: Physical - Deal 2 damage to an enemy. Energy - Deal 1 damage to each character. Mental - Discard Hulk. Wild - All of the above.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_050.png",
        card_back_image_path: PLAYER_CARD_BACK_ASSET.path,
        card_amount_max: 1,
    })
}
