use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, SupportCard,
};

pub fn get_stark_tower() -> Card {
    Card::Support(SupportCard {
        id: "core_034",
        name: "Stark Tower",
        description:
            "Alter-Ego Action: Exhaust Stark Tower -> choose a player. That player returns the topmost Tech upgrade in their discard pile to their hand.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_034.png",
        traits: vec![CardTrait::Location],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 2,
        res: vec![CardResource::Mental],
        card_amount_max: 1,
    })
}
