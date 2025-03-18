use crate::features::cards::{Card, TreacheryCard};

pub fn get_false_alarm() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_112",
        name: "False Alarm",
        description:
            "When Revealed: You are confused. If you are already confused, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/bomb_scare/core_112.png",
        boost: 1,
        traits: vec![],
        keywords: vec![],
        boost_effect: None,
    })
}
