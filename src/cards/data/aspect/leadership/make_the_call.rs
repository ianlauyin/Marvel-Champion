use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_071",
        name: "Make the Call",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Event,
        CardCost::constant(0),
        CardResources::mental(),
        CardTraits::single(CardTrait::Tactic),
        InstantAbilities::single(Ability::new(instant_ability)),
    )).id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
