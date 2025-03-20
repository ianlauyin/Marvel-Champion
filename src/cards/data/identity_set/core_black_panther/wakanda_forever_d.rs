use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands, world::World};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_043d",
        name: "Wakanda Forever!",
        sub_name: None,
        unique: false,
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
            PlayerCardType::Event,
            CardCost::constant(1),
            CardResources::wild(),
            CardTraits::single(CardTrait::Tactic),
            InstantAbilities::single(Ability::hero(instant_ability)),
        ))
        .id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
