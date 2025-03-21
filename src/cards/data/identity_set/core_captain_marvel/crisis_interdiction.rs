use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_012",
        name: "Crisis Interdiction",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::IdentitySet(IdentitySet::CoreCaptainMarvel).into(),
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
            CardCost::constant(2),
            CardResources::energy(),
            CardTraits::single(CardTrait::Thwart),
            CardFormLimit::hero(),
        ))
        .id()
}
