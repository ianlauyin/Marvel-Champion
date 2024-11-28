use crate::features::cards::{
    Card, CardAspect, CardResource, CardTrait, Identity::CoreIronMan, SupportCard,
};

pub fn get_pepper_potts() -> Card {
    Card::Support(SupportCard {
        id: "core_033",
        name: "Pepper Potts",
        description:
            "Resource: Exhaust Pepper Potts -> generate the resources of the top card in your discard pile.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_iron_man/core_033.png",
        traits: vec![CardTrait::Persona],
        keywords: vec![],
        card_icons: vec![],
        aspect: CardAspect::IdentitySpecific(CoreIronMan),
        unique: true,
        cost: 3,
        res: vec![CardResource::Physical],
        card_amount_max: 1,
    })
}
