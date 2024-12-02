use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_vibranium_armor() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_152",
        name: "Vibranium Armor",
        description:
            "Attach to the villain. Forced Response: After the villain take damage, give it a tough status card. Hero Action: Exhaust your hero and spend 2 Physical resources -> discard this card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/under_attack/core_152.png",
        boost: 1,
        traits: vec![CardTrait::Armor],
        card_icons: vec![],
        atk_modifier: 0,
        sch_modifier: 0,
        keywords:vec![],
    })
}
