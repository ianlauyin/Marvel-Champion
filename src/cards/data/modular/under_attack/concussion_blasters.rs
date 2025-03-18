use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_concussion_blasters() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_153",
        name: "Concussion Blasters",
        description:
            "Attach to the villain. The villain gains retaliate 1. Hero Action: Exhaust your hero and spend 2 Energy resources -> discard this card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/under_attack/core_153.png",
        boost: 1,
        traits: vec![CardTrait::Weapon],
        card_icons: vec![],
        atk_modifier: 1,
        sch_modifier: 0,
        keywords: vec![],
        boost_effect:None,
    })
}
