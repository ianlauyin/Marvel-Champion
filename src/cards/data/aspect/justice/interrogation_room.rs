use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_063",
        name: "Interrogation Room",
        sub_name: None,
        unique: false,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Justice).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Support,
        CardCost::constant(1),
        CardResources::energy(),
        CardTraits::single(CardTrait::Location),
        ResponseAbilities::single(Ability::new(response_ability)),
    )).id()
}

fn response_ability(world: &mut World) {
    println!("response_ability");
}
