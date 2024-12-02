use crate::features::cards::{Card, TreacheryCard};

pub fn get_whiplash() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_173",
        name: "Electric Whip Attack",
        boost: 0,
        description: "When Revealed: Choose to either deal 1 damage to your hero for each upgrade you control or choose and discard an upgrade you control. Boost: If the villain is making an undefended attack, choose and discard an upgrade you control. ",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_173.png",
        traits: vec![],
        keywords: vec![],
    })
}
