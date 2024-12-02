use crate::features::cards::{AttachmentCard, Card, CardTrait, Keyword};

pub fn get_biomechanical_upgrades() -> Card {
    Card::Attachment(AttachmentCard {
        id: "core_185",
        name: "Biomechanical Upgrades",
        description:
            "Surge. Attach to the minion with the highest printed hit points and without another Biomechanical Upgrades attached. Forced Interrupt: When attached minion would be defeated, heal all damage from it instead, then discard this card.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/the_doomsday_chair/core_185.png",
        boost: 1,
        traits: vec![CardTrait::Tech],
        card_icons: vec![],
        atk_modifier:0,
        sch_modifier: 0,
        keywords:vec![Keyword::Surge],
        boost_effect:None,
    })
}
