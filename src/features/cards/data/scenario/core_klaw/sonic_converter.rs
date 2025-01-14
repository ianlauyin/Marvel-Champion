use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_sonic_converter() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_118",
        name: "Sonic Converter",
        description: "Attach to Klaw. Forced Response: After Klaw attacks and damages a character, stun that character. Hero Action: Spend Energy Physical Mental resources -> discard this card",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_klaw/core_118.png",
        card_icons: vec![],
        boost: 3,
        traits: vec![CardTrait::Weapon],
        atk_modifier: 1,
        sch_modifier: 0,
        keywords:vec![],
        boost_effect:None,
    })
}
