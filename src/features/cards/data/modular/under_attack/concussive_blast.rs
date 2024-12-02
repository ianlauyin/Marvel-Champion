use crate::features::cards::{Card, TreacheryCard};

pub fn get_concussive_blast() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_154",
        name: "Concussive Blast",
        description:
            "When Revealed: Deal 1 damage to each friendly character. Boost: Deal 1 damage to each character you control.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/under_attack/core_154.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
