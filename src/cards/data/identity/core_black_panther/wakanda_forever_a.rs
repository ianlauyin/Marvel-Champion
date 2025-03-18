use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreBlackPanther,
};

pub fn get_wakanda_forever_a() -> Card {
    Card::Event(EventCard {
        id: "core_043a",
        name: "Wakanda Forever!",
        description:"Hero Action: Resolve the \"Special\" ability on each Black Panther upgrade you control in any order. (Resolving each ability is a step in a sequence.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_043a.png",
        traits: vec![CardTrait::Tactic],
        keywords: vec![],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        cost: 1,
        res: vec![CardResource::Energy],
        card_amount_max: 1,
    })
}
