use crate::features::cards::{Card, TreacheryCard};

pub fn get_swarm_attack() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_147",
        name: "Swarm Attack",
        traits: vec![],
        description: "When Revealed: Each Drone minion engaged with your hero attacks. If no attack was made this way, put the top card of your deck into play facedown, engaged with you as a Drone minion.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_ultron/core_147.png",
        boost: 1,
        keyword: vec![],
    })
}
