use crate::features::cards::{Card, TreacheryCard};

pub fn get_sweeping_swoop() -> Card {
    Card::Treachery(TreacheryCard {
        id: "core_168",
        name: "Sweeping Swoop",
        traits: vec![],
        boost: 0,
        description: "When Revealed: Stun your hero. If Vulture is in play, this card gains surge.
        Boost: If this activation deals damage to a friendly character, stun that character.",
        abilities: vec![],
        search_keywords: vec![],
        card_image_path: "embedded://cards/identity_specific_card/core_spider_man/core_168.png",
        card_back_image_path: "embedded://cards/card_backs/encounter_card_back.png",
    })
}
