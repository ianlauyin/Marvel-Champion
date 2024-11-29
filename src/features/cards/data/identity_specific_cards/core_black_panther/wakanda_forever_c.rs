use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, EventCard, Identity::CoreBlackPanther,
};

pub fn get_wakanda_forever_c() -> Card {
    Card::Event(EventCard {
        id: "core_043c",
        name: "Wakanda Forever!",
        description:"Hero Action: Resolve the \"Special\" ability on each Black Panther upgrade you control in any order. (Resolving each ability is a step in a sequence.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_black_panther/core_043c.png",
        traits: vec![CardTrait::Tactic],
        keywords: vec![],
        aspect: CardAspect::IdentitySpecific(CoreBlackPanther),
        cost: 1,
        res: vec![CardResource::Physical],
        card_amount_max: 1,
    })
}
