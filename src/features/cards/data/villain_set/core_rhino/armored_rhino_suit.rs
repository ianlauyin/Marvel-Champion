use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_armored_rhino_suit() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_098",
        name: "Armored Rhino Suit",
        description: "Attach to Rhino. Forced Interrupt: When any amount of damage would be dealt to Rhino, place it here instead. Then, if there is at least 5 damage here, discard Armored Rhino Suit.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_098.png",
        card_icons: vec![],
        boost: 0,
        traits: vec![CardTrait::Armor],
        atk_modifier: 0,
        sch_modifier: 0,
        keywords:vec![],
    })
}
