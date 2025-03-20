use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands, world::World};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_047",
        name: "Panther Claws",
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
            PlayerCardType::Upgrade,
            CardCost::constant(2),
            CardResources::energy(),
            CardTraits::new(vec![CardTrait::Weapon, CardTrait::BlackPanther]),
            InstantAbilities::single(Ability::new(instant_ability)),
        ))
        .id()
}

fn instant_ability(world: &mut World) {
    println!("instant_ability");
}
