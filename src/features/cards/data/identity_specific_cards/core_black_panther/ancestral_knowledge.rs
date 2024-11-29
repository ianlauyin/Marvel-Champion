use crate::features::cards::{
    Card, CardAspect, CardResource, EventCard, Identity::CoreBlackPanther,
};

pub fn get_ancestral_knowledge() -> Card {
    Card::Event(EventCard {
        id: "core_042",
        name: "Ancestral Knowledge",
        description:"Alter-Ego Action: Choose up to 3 different cards in your discard pile and shuffle them into your deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_042.png",
        traits: vec![],
        keywords: vec![],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        cost: 1,
        res: vec![CardResource::Mental],
        card_amount_max: 1,
    })
}
