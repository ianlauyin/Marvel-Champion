use crate::{
    constants::PLAYER_CARD_BACK_PATH,
    features::cards::{Card, CardAspect, CardResource, CardTrait, Counter, Keyword, SupportCard},
};
pub fn get_tac_team() -> Card {
    Card::Support(SupportCard {
        id: "core_056",
        name: "Tac Team",
        unique: false,
        cost: 3,
        card_icons: vec![],
        aspect: CardAspect::Aggression,
        res: vec![CardResource::Energy],
        keywords: vec![Keyword::Use(3, Counter::Attack)],
        traits: vec![CardTrait::SHIELD],
        description: "Uses (3 attack counters). (Enters play with 3 counters. When those are gone, discard this card) Action: Exhaust Tac Team and remove 1 attack counter from it -> deal 2 damage to an enemy.",
        abilities: vec![],
        card_image_path: "embedded://cards/aggression/core_056.png",
        card_back_image_path: PLAYER_CARD_BACK_PATH,
        card_amount_max: 2,
    })
}
