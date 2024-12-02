use crate::features::cards::{Card, TreacheryCard};

pub fn get_yon_roggs_treason() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_179",
        name: "Yon-Rogg's Treason",
        boost: 1,
        description:"When Revealed: Discard each Energy resource from your hand. If you discarded no cards this way, this card gains surge.",
        abilities: vec![],
        keywords: vec![],
        card_image_path: "embedded://cards/identity/core_captain_marvel/core_179.png",
        traits: vec![],
    })
}
