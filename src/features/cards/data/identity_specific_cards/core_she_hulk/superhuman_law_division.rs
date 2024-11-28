use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreSheHulk, SupportCard,
};

pub fn get_superhuman_law_division() -> Card {
    Card::Support(SupportCard {
        id: "core_026",
        name: "Superhuman Law Division",
        card_icons: vec![],
        description: "Alter-Ego Action (thwart): Exhaust Superhuman Law Division and spend a  resource → remove 2 threat from a scheme.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_026.png",
        keywords: vec![],
        traits: vec![CardTrait::Location],
        aspect: CardAspect::IdentitySpecific(CoreSheHulk),
        unique: false,
        cost: 1,
        res: vec![CardResource::Physical],
        card_amount_max: 1,
    })
}
