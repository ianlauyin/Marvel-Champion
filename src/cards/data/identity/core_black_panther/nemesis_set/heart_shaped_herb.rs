use bevy::ecs::world::World;

use crate::{
    cards::{set::IdentitySet, Belong},
    component::{
        ability::InstantAbilities,
        card::{CardBasic, CardBoost, CardKeyword, CardKeywords},
    },
};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_158",
        name: "Heart-Shaped Herb",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(&mut World)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(world: &mut World) {
    let keywords: CardKeywords = CardKeyword::Surge.into();
    let instant_abilities = InstantAbilities::single(instant_ability);
    world.spawn((get_info(), CardBoost::new(0), keywords, instant_abilities));
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
