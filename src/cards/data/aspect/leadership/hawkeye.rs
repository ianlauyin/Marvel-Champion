use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_066",
        name: "Hawkeye",
        sub_name: Some("Clint Barton"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Ally,
        CardCost::constant(3),
        CardResources::energy(),
        CardTraits::single(CardTrait::Avenger),
        CardCharacter::ally(3, 1, 1, 1, 1),
        CardCounter::Arrow(4),
        ResponseAbilities::single(Ability::new(response_ability)),
    )).id()
}

fn response_ability(world: &mut World) {
    println!("response_ability");
}
