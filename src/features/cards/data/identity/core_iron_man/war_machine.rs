use crate::features::cards::{
    AllyCard, Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan,
};

pub fn get_war_machine() -> Card {
    Card::Ally(AllyCard {
        id: "core_030",
        name: "War Machine",
        description:
            "Action: Exhaust War Machine and deal 2 damage to him → deal 1 damage to each enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_030.png",
        traits: vec![CardTrait::SHIELD, CardTrait::Soldier],
        initial_hit_points: 4,
        keywords: vec![],
        card_icons: vec![],
        sub_name: "James Rhodes",
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 4,
        res: vec![CardResource::Wild],
        thw: 1,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        card_amount_max: 1,
    })
}
