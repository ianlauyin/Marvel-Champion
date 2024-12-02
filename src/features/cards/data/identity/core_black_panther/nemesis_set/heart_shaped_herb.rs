use crate::features::cards::{Card, Keyword, TreacheryCard};

pub fn get_heart_shaped_herb() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_158",
        name: "Heart-Shaped Herb",
        boost: 0,
        description: "Surge (After this card resolves, reveal 1 additional encounter card) When Revealed: Give the villain and each minion engaged with you a tough status card.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_158.png",
        traits: vec![],
        keywords:vec![Keyword::Surge],
    })
}
