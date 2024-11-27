use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, Counter, Keyword, SupportCard},
};
pub fn get_med_team() -> Card {
    Card::Support(SupportCard {
        id: "core_080",
        name: "Med Team",
        aspect: CardAspect::Protection,
        res: vec![CardResource::Energy],
        traits: vec![],
        keywords: vec![Keyword::Use(3, Counter::Medical)],
        description: "Uses (3 medical counters). (Enters play with 3 counters. When those are gone, discard this card.) Action: Exhaust Med Team and remove 1 medical counter from it -> heal 2 damage from a friendly character.",
        abilities: vec![],
        card_image_path: "embedded://cards/protection/core_080.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 3,
        unique: false,
        cost: 3,
        card_icons: vec![],
    })
}
