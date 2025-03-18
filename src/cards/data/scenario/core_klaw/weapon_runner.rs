use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_weapon_runner() -> Card {
    Card::Minion(MinionCard {
        id: "core_121",
        name: "Weapons Runner",
        description: "Surge. (After this card is revealed, reveal 1 additional encounter card.) Boost: Put Weapons Runner into play engaged with you.",
        abilities: vec![],
        card_image_path: "embedded://cards/scenario/core_klaw/core_121.png",
        card_icons: vec![],
        boost: 0,
        traits: vec![CardTrait::Mercenary],
        unique: false,
        initial_hit_points: 2,
        keywords: vec![Keyword::Surge],
        sch: 1,
        atk: 1,
        boost_effect:None,
    })
}
