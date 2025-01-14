use crate::features::cards::{Card, TreacheryCard};

pub fn get_rage_of_ultron() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_145",
        name: "Rage of Ultron",
        traits: vec![],
        description: "When Revealed (Alter-Ego): Ultron schemes. Discard the top card of your deck for each threat placed this way. When Revealed (Hero): Ultron attacks you. Discard the top card of your deck for each damage dealt by this attack.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_145.png",
        boost: 2,
        keywords: vec![],
        boost_effect:None,
    })
}
