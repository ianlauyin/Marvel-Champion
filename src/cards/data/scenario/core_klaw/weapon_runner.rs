use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_121",
        name: "Weapons Runner",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::Scenario(Scenario::CoreKlaw).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::Minion,
            CardBoost::new(0),
            CardCharacter::minion(2, 1, 1),
            CardKeywords::single(CardKeyword::Surge),
            CardTraits::single(CardTrait::Mercenary),
            BoostAbilities::single(Ability::new(boost_ability)),
        ))
        .id()
}

fn boost_ability(world: &mut World) {
    println!("boost_ability");
}

// pub fn get_weapon_runner() -> Card {
//     Card::Minion(MinionCard {
//         id: "core_121",
//         name: "Weapons Runner",
//         description: "Surge. (After this card is revealed, reveal 1 additional encounter card.) Boost: Put Weapons Runner into play engaged with you.",
//         abilities: vec![],
//         card_image_path: "embedded://cards/scenario/core_klaw/core_121.png",
//         card_icons: vec![],
//         boost: 0,
//         traits: vec![CardTrait::Mercenary],
//         unique: false,
//         initial_hit_points: 2,
//         keywords: vec![Keyword::Surge],
//         sch: 1,
//         atk: 1,
//         boost_effect:None,
//     })
// }
