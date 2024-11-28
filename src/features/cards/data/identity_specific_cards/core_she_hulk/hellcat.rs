use crate::features::cards::{
    AllyCard, Card, CardAspect, CardResource, CardTrait, Identity::CoreSheHulk,
};

pub fn get_hellcat() -> Card {
    Card::Ally(AllyCard {
        id: "core_020",
        name: "Hellcat",
        card_icons: vec![],
        description: "Action: Return Hellcat to your hand.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_020.png",
        initial_hit_points: 3,
        keywords: vec![],
        traits: vec![CardTrait::Avenger],
        thw: 2,
        atk: 1,
        sub_name: "Patsy Walker",
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        unique: true,
        cost: 3,
        res: vec![CardResource::Wild],
        thw_con_dmg: 2,
        atk_con_dmg: 1,
        card_amount_max: 1,
    })
}
