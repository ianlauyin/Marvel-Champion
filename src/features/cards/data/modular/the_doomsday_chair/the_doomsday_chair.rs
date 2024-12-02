use crate::features::cards::{Card, CardIcon, Count, SideSchemeCard};

pub fn get_the_doomsday_chair() -> Card {
    Card::SideScheme(SideSchemeCard {
        id: "core_183",
        name: "The Doomsday Chair",
        description:
            "When Revealed: If MODOK is not in play, search the encounter deck and discard pile for MODOK and put him into play engaged with you, then shuffle the encounter deck.",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/the_doomsday_chair/core_183.png",
        boost: 3,
        traits: vec![],
        card_icons: vec![CardIcon::Acceleration],
        initial_threat: Count::Constant(8),
    })
}
