use crate::features::cards::{Card, TreacheryCard};

pub fn get_android_efficiency_c() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_144c",
        name: "Android Efficiency",
        traits: vec![],
        description: "When Revealed: Each player puts the top card of their deck into play facedown, engaged with them as a Drone minion. Boost: Choose to either spend a Physical resource or put the top card of the deck into play facedown, engaged with you as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_144c.png",
        boost: 0,
        keywords: vec![],
        boost_effect:None,
    })
}
