use crate::features::cards::{Card, TreacheryCard};

pub fn get_im_tough() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_105",
        name: "\"I'm Tough\"",
        description: "When Revealed: Give Rhino a tough status card. If Rhino already has a tough status card, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_rhino/core_105.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
