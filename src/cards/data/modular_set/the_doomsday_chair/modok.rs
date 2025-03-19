use crate::features::cards::{Card, CardTrait, Keyword, MinionCard};

pub fn get_modok() -> Card {
    Card::Minion(MinionCard {
        id: "core_184",
        name: "MODOK",
        description:
            "Retaliate 2. (After this character is attacked, deal 2 damage to the attacking character.)",
        abilities: vec![],
        card_image_path: "embedded://cards/modular/the_doomsday_chair/core_184.png",
        boost: 2,
        traits: vec![CardTrait::Cyborg,CardTrait::Elite],
        card_icons: vec![],
        unique: true,
        initial_hit_points: 8,
        keywords: vec![Keyword::Retaliate(2)],
        sch: 2,
        atk: 2,
        boost_effect:None,
    })
}
