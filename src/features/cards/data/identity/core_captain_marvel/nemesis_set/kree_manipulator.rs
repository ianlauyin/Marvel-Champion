use crate::features::cards::{Card, Keyword, TreacheryCard};

pub fn get_kree_manipulator() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_178",
        name: "Kree Manipulator",
        boost: 0,
        description:"Surge. (After this card resolves, reveal 1 additional encounter card.) When Revealed: Place 1 threat on the main scheme.",
        abilities: vec![],
        keywords: vec![Keyword::Surge],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_178.png",
        traits: vec![],
        boost_effect:None,
    })
}
