use crate::features::cards::{Card, TreacheryCard};

pub fn get_ritual_combat() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_159",
        name: "Ritual Combat",
        boost: 2,
        description: "When Revealed: Discard the top card of the encounter deck. Then, choose to either deal X damage to your hero or place X threat on the main scheme. X is 1 more than the number of boost icons on the discarded encounter card.",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_black_panther/core_159.png",
        traits: vec![],
        keyword:vec![],
    })
}
