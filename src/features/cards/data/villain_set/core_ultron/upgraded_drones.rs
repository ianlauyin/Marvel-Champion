use crate::features::cards::{AttachmentCard, Card, CardTrait};

pub fn get_upgraded_drones() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_142",
        name: "Upgraded Drones",
        traits: vec![CardTrait::Condition],
        card_icons: vec![],
        description: "Attach to the Ultron Drones environment. Each facedown Drone minion gets +1 ATK and +1 hit point. Hero Action: Spend Energy Mental Physical resources -> discard this card",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_142.png",
        boost: 0,
        atk_modifier: 0,
        sch_modifier: 0,
    })
}
