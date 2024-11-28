use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait};
pub fn get_vision() -> Card {
    Card::Ally(AllyCard {
        id: "core_068",
        name: "Vision",
        sub_name: "Vision",
        unique: true,
        cost: 4,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Physical],
        traits: vec![CardTrait::Android, CardTrait::Avenger],
        initial_hit_points: 3,
        thw: 1,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Action: Spend a Energy resource -> choose THW or ATK. Until the end of the phase, Vision gets +2 to the chosen power. (Limit once per round.)",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_068.png",
        card_amount_max: 1,
    })
}
