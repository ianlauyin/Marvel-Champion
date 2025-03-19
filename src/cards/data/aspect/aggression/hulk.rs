use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_050",
        name: "Hulk",
        sub_name: Some("Bruce Banner"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Ally,
        CardCost::constant(2),
        CardResources::energy(),
        CardTraits::new(vec![CardTrait::Avenger, CardTrait::Gamma]),
        CardCharacter::ally(5, 0, 0, 3, 1),
        ForcedResponseAbilities::single(Ability::new(forced_response_ability)),
    ));
}

fn forced_response_ability(world: &mut World) {
    println!("forced_response_ability");
}
