use crate::features::cards::{Card, TreacheryCard};

pub fn get_android_efficiency_b() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_144b",
        name: "Android Efficiency",
        traits: vec![],
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Boost: Choose to either spend a Mental resource or put the top card of the deck into play facedown, engaged with you as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_ultron/core_144b.png",
        boost: 0,
        keywords: vec![],
        boost_effect:None,
    })
}
