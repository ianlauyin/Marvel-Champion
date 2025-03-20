use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_060",
        name: "For Justice!",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Justice).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Event,
        CardCost::constant(2),
        CardResources::energy(),
        CardTraits::single(CardTrait::Thwart),
        CardFormLimit::hero(),
        InstantAbilities::single(Ability::hero(instant_ability)),
    )).id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
