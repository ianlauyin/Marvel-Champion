use crate::features::cards::{Card, TreacheryCard};

pub fn get_master_of_mayhem() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_133",
        name: "Masters of Mayhem",
        description:
            "When Revealed: Each Masters of Evil minion attacks the hero it is engaged with. If no attacks were made this way, search the encounter deck and discard pile for a Masters of Evil minion and put it into play engaged with you, then shuffle the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_133.png",
        boost: 2,
        traits: vec![],
        keywords: vec![],
    })
}
