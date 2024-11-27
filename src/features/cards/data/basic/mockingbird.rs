use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait},
};
pub fn get_mockingbird() -> Card {
    Card::Ally(AllyCard {
        id: "core_083",
        name: "Mockingbird",
        sub_name: "Bobbi Morse",
        aspect: CardAspect::Basic,
        unique: true,
        cost: 3,
        res: vec![CardResource::Physical],
        initial_hit_points: 3,
        keywords: vec![],
        traits: vec![CardTrait::SHIELD, CardTrait::Spy],
        card_icons: vec![],
        thw: 1,
        thw_con_dmg: 1,
        atk: 1,
        atk_con_dmg: 1,
        description: "Response: After Mockingbird enters play, stun an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/basic/core_083.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 1,
    })
}
