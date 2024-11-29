use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_whiplash() -> Card {
    Card::Minion(MinionCard {
        id: "core_172",
        name: "Whiplash",
        boost: 2,
        card_icons: vec![],
        description: "Retaliate 1 (After this character is attacked, deal 1 damage to the attacking character.)",
        abilities: vec![],
        card_image_path: "embedded://cards/identity/core_iron_man/core_172.png",
        unique: true,
        initial_hit_points: 4,
        keywords: vec![Keyword::Retaliate],
        traits: vec![CardTrait::Criminal],
        sch: 2,
        atk: 3,
    })
}
