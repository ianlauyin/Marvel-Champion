use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_033",
        name: "Pepper Potts",
        sub_name: None,
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreIronMan).into(),
        is_vertical: true,
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
            CardCost::constant(3),
            CardResources::physical(),
            CardTraits::single(CardTrait::Persona),
        ))
        .id()
}
