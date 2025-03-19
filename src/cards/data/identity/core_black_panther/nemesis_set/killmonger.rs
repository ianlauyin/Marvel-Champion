use bevy::ecs::{system::Commands, world::World};

use crate::{
    cards::{Belong, IdentitySet},
    component::{
        ability::ConstantAbilities,
        card::{CardBasic, CardBoost, CardCharacter, CardTrait, CardTraits},
    },
};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_157",
        name: "Killmonger",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        CardBoost::new(2),
        CardTraits::new(vec![
            CardTrait::Assassin,
            CardTrait::Elite,
            CardTrait::Mercenary,
        ]),
        CardCharacter::minion(5, 2, 2),
        ConstantAbilities::single(constant_ability),
    ));
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}
