use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_enhanced_ivory_horn() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_100",
        name: "Enhanced Ivory Horn",
        description:
            "Attach to Rhino. Hero Action: Spend 3 Physical resources -> discard this card",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_100.png",
        card_icons: vec![],
        boost: 2,
        traits: vec![CardTrait::Weapon],
        atk_modifier: 1,
        sch_modifier: 0,
    })
}
