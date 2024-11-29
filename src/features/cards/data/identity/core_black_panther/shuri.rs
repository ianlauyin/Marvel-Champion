use crate::features::cards::{
    AllyCard, Card, CardAspect, CardResource, CardTrait, Identity::CoreBlackPanther,
};

pub fn get_shuri() -> Card {
    Card::Ally(AllyCard {
        id: "core_041",
        name: "Shuri",
        description:"Response: After Shuri enters play, search your deck for an upgrade and add it to your hand. Shuffle your deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_041.png",
        traits: vec![CardTrait::Genius,CardTrait::Wakanda],
        initial_hit_points: 3,
        keywords: vec![],
        card_icons: vec![],
        thw: 1,
        atk: 1,
        sub_name: "Shuri",
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        unique: true,
        cost: 2,
        res: vec![CardResource::Physical],
        thw_con_dmg: 1,
        atk_con_dmg: 1,
        card_amount_max: 1,
    })
}
