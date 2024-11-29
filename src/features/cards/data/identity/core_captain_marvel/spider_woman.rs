use crate::features::cards::{
    AllyCard, Card, CardAspect, CardResource, CardTrait, Identity::CoreCaptainMarvel,
};
pub fn get_spider_woman() -> Card {
    Card::Ally(AllyCard {
        id: "core_011",
        name: "Spider-Woman",
        sub_name: "Jessica Drew",
        aspect: CardAspect::IdentitySpecific(CoreCaptainMarvel),
        unique: true,
        cost: 3,
        res: vec![CardResource::Wild],
        initial_hit_points: 2,
        keywords: vec![],
        traits: vec![CardTrait::Avenger, CardTrait::Spy],
        card_icons: vec![],
        thw: 2,
        thw_con_dmg: 1,
        atk: 2,
        atk_con_dmg: 1,
        description: "Response: After Spider-Woman enters play, confuse the villain.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_011.png",
        card_amount_max: 1,
    })
}
