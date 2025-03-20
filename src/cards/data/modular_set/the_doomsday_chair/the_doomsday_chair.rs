use bevy::ecs::{entity::Entity, system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_183",
        name: "The Doomsday Chair",
        sub_name: None,
        unique: false,
        card_amount_max: 2,
        belongs: Belong::ModularSet(ModularSet::TheDoomsdayChair).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands
        .spawn((
            get_info(),
            EncounterCardType::SideScheme,
            CardBoost::new(3),
            CardScheme::new(Count::Constant(8)),
            CardIcons::single(CardIcon::Acceleration),
            WhenRevealedAbilities::single(Ability::new(when_revealed)),
        ))
        .id()
}

fn when_revealed(world: &mut World) {
    println!("when_revealed");
}
