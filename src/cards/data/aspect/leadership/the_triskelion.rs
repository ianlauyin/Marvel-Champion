use bevy::ecs::{system::Commands, world::World};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_073",
        name: "The Triskelion",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Support,
        CardCost::constant(1),
        CardResources::energy(),
        ConstantAbilities::single(Ability::new(constant_ability)),
    ));
}

fn constant_ability(world: &mut World) {
    println!("constant_ability");
}
