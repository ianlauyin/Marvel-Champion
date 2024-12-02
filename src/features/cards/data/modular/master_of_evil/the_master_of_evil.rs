use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_the_master_of_evil() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_128",
        name: "The Masters of Evil",
        description:
            "When Revealed: Discard cards from the encounter deck until a Masters of Evil minion is discarded. Put that minion into play engaged with the first player.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/master_of_evil/core_128.png",
        boost: 2,
        traits: vec![],
        card_icons: vec![CardIcon::Acceleration],
        initial_threat: Count::PerPlayer(3),
    })
}
