use crate::features::cards::{Card, TreacheryCard};

pub fn get_shadow_of_the_past() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_190",
        name: "Gang-Up",
        description: "When Revealed: Reveal your set-aside nemesis minion and put it into play engaged with you. Reveal your set-aside nemesis side scheme and put it into play. Shuffle the rest of your set-aside nemesis encounter set into the encounter deck. If your nemesis minion does not enter the game this way, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/standard/core_190.png",
        boost: 2,
        traits: vec![],
        keywords: vec![],
    })
}
