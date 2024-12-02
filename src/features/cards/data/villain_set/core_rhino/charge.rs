use crate::features::cards::{AttachmentCard, Card};

pub fn get_charge() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_099",
        name: "Charge",
        description: "Attach to Rhino. Forced Interrupt: When Rhino attacks, the attack gains overkill. (Excess damage to an ally from this attack is dealt to that ally's controller.) At the end of this attack, discard Charge.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_099.png",
        card_icons: vec![],
        boost: 2,
        traits: vec![],
        atk_modifier: 3,
        sch_modifier: 0,
        keywords:vec![],
    })
}
