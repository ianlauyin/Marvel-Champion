use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait};
pub fn get_daredevil() -> Card {
    Card::Ally(AllyCard {
        id: "core_058",
        name: "Daredevil",
        sub_name: "Matt Murdock",
        unique: true,
        cost: 4,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Justice,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Defender],
        initial_hit_points: 3,
        thw: 2,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Response: After Daredevil thwarts, deal 1 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/justice/core_058.png",
        card_amount_max: 1,
    })
}
