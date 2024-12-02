use crate::features::cards::{Card, TreacheryCard};

pub fn get_explosion() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_111",
        name: "Explosion",
        description:
            "When Revealed: If Bomb Scare is in play, assign X damage among heroes and allies, where X is the amount of threat on Bomb Scare. If Bomb Scare is not in play, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/bomb_scare/core_111.png",
        boost: 2,
        traits: vec![],
        keywords: vec![],
    })
}
