use crate::features::cards::{Card, TreacheryCard};

pub fn get_titanias_fury() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_164",
        name: "Titania's Fury",
        boost: 1,
        description: "When Revealed: Titania attacks your hero. If Titania did not attack, heal all damage from Titania and this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_she_hulk/core_164.png",
        traits: vec![],
        keywords: vec![],
    })
}
