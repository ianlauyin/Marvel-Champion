use crate::features::cards::{AllyCard, Card, CardAspect, CardResource, CardTrait};
pub fn get_hawkeye() -> Card {
    Card::Ally(AllyCard {
        id: "core_066",
        name: "Hawkeye",
        sub_name: "Clint Barton",
        unique: true,
        cost: 3,
        card_icons: vec![],
        keywords: vec![],
        aspect: CardAspect::Leadership,
        res: vec![CardResource::Energy],
        traits: vec![CardTrait::Avenger],
        initial_hit_points: 3,
        thw: 1,
        thw_con_dmg: 1,
        atk: 1,
        atk_con_dmg: 1,
        description: "Hawkeye enters play with 4 arrow counters on him. Response: After a minion enters play, remove 1 arrow counter from Hawkeye -> deal 2 damage to that minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/leadership/core_066.png",
        card_amount_max: 1,
    })
}
