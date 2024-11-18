use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreSpiderMan,
};
pub fn get_enhanced_spider_sense() -> Card {
    Card::Event(EventCard {
        id: "core_004",
        name: "Enhanced Spider-Sense",
        aspect: CardAspect::IdentitySpecific(CoreSpiderMan),
        cost: 1,
        res: vec![CardResource::Mental],
        traits: vec![CardTrait::Superpower],
        description: "Hero Interrupt: When a treachery card is revealed from the encounter deck, cancel its \"When Revealed\" effects.",
        abilities: vec![],
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_004.png",
        card_back_image_path: "embedded://cards/card_backs/player_card_back.png",
        card_amount_max: 2,
    })
}
