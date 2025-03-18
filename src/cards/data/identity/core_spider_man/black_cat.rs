use crate::features::cards::{
    AllyCard, Card, CardAspect, CardResource, CardTrait, Identity::CoreSpiderMan,
};
pub fn get_black_cat() -> Card {
    Card::Ally(AllyCard {
        id: "core_002",
        name: "Black Cat",
        sub_name: "Felicia Hardy",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        unique: true,
        cost: 2,
        res: vec![CardResource::Energy],
        initial_hit_points: 2,
        keywords: vec![],
        traits: vec![CardTrait::HeroForHire],
        card_icons: vec![],
        thw: 1,
        thw_con_dmg:1,
        atk: 1,
        atk_con_dmg: 0,
        description: "Forced Response: After you play Black Cat, discard the top 2 cards of your deck. Add each card with a printed mental resource discarded this way to your hand.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_spider_man/core_002.png",
        card_amount_max: 1,
    })
}
