use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait};
pub fn get_jessica_jones() -> Card {
    Card::Ally(AllyCard {
        id: "core_059",
        name: "Jessica Jones",
        sub_name: "Jessica Jones",
        unique: true,
        cost: 3,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Defender],
        initial_hit_points: 3,
        thw: 1,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Jessica Jones gets +1 THW for each side scheme in play.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_059.png",
        card_amount_max: 1,
    })
}
