use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait};
pub fn get_maria_hill() -> Card {
    Card::Ally(AllyCard {
        id: "core_067",
        name: "Maria Hill",
        sub_name: "Maria Hill",
        unique: true,
        cost: 2,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::SHIELD],
        initial_hit_points: 2,
        thw: 2,
        thw_con_dmg: 1,
        atk: 1,
        atk_con_dmg: 1,
        description: "Response: After Maria Hill enters play, each player draws 1 card.",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_067.png",
        card_amount_max: 1,
    })
}
