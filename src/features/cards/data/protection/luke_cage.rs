use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait, Keyword};
pub fn get_luke_cage() -> Card {
    Card::Ally(AllyCard {
        id: "core_076",
        name: "Luke Cage",
        sub_name: "Luke Cage",
        unique: true,
        cost: 4,
        card_icons: vec![],
        keywords: vec![Keyword::Toughness],
        aspect: CardAspect::Protection,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Defender],
        initial_hit_points: 5,
        thw: 1,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Toughness. (This character enters play with a tough status card.)",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_076.png",
        card_amount_max: 1,
    })
}
