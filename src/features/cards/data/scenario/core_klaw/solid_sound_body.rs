use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_solid_sound_body() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_119",
        name: "Solid-Sound Body",
        description: "Attach to Klaw. Klaw gains retaliate 1. (After this character is attacked, deal 1 damage to the attacking character.) Hero Action: Spend Energy Mental Physical resources -> discard this card.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_119.png",
        card_icons: vec![],
        boost: 3,
        traits: vec![CardTrait::Condition],
        atk_modifier: 0,
        sch_modifier: 0,
        keywords:vec![],
        boost_effect:None,
    })
}
