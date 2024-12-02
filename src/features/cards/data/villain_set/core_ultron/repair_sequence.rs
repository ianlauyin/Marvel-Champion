use crate::features::cards::{Card, TreacheryCard};

pub fn get_repair_sequence() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_146",
        name: "Repair Sequence",
        traits: vec![],
        description: "When Revealed: Ultron heals 2 damage for each Drone minion engaged with you. If no damage was healed this way, this card gains surge. Boost: Ultron heals 1 damage for each Drone minion engaged with you.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_146.png",
        boost: 1,
        keywords: vec![],
        boost_effect:None,
    })
}
