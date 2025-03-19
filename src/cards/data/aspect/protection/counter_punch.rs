use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_077",
        name: "Counter-Punch",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Protection).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Event,
        CardCost::constant(0),
        CardResources::physical(),
        CardTraits::single(CardTrait::Attack),
        ResponseAbilities::single(Ability::new(response_ability)),
    ));
}

fn response_ability(world: &mut World) {
    println!("response_ability");
}
