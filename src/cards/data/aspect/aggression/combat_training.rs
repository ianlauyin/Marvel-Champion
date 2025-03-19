use bevy::ecs::system::Commands;

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_057",
        name: "Combat Training",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Aggression).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands)) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) {
    commands.spawn((
        get_info(),
        PlayerCardType::Upgrade,
        CardCost::constant(2),
        CardResources::physical(),
        CardTraits::single(CardTrait::Skill),
        StatsModifier::new(0, 1, 0),
    ));
}
