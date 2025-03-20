use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands, world::World};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_045",
        name: "The Golden City",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            PlayerCardType::Support,
            CardCost::constant(2),
            CardResources::energy(),
            CardTraits::new(vec![CardTrait::Location, CardTrait::Wakanda]),
            InstantAbilities::single(Ability::alter_ego(instant_ability)),
        ))
        .id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
