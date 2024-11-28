use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_genetically_enhanced() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_163",
        name: "Genetically Enhanced",
        boost: 1,
        card_icons: vec![],
        description: "Attach to the minion with the highest printed hit points. If there are no minions in play, this card gains surge. Attached minion gets +3 hit points.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_she_hulk/core_163.png",
        unique: false,
        traits: vec![CardTrait::Condition],
    })
}
