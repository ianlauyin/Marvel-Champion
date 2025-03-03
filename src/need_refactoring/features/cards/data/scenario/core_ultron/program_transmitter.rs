use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_program_transmitter() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_141",
        name: "Program Transmitter",
        traits: vec![CardTrait::Item,CardTrait::Tech],
        card_icons: vec![],
        description: "Attach to Ultron. Forced Response: After Ultron schemes, place 1 threat on each side scheme. Hero Action: Exhaust your hero and spend 2 Mental resources -> discard this card.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_ultron/core_141.png",
        boost: 1,
        atk_modifier: 0,
        keywords:vec![],
        sch_modifier: 1,
        boost_effect:None,
    })
}
