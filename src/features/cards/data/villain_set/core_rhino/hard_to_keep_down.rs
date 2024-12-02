use crate::features::cards::{Card, TreacheryCard};

pub fn get_hard_to_keep_down() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_104",
        name: "Hard to Keep Down",
        description: "When Revealed: Rhino heals 4 damage. If no damage was healed this way, this card gains surge.",
        abilities: vec![],
        card_image_path: "embedded://cards/villain/core_rhino/core_104.png",
        boost: 0,
        traits: vec![],
        keywords: vec![],
        boost_effect:None,
    })
}
