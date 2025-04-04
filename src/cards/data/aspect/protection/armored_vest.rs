use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_081",
        name: "Armored Vest",
        sub_name: None,
        unique: false,
        card_amount_max: 3,
        belongs: Belong::Aspect(Aspect::Protection).into(),
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
            PlayerCardType::Upgrade,
            CardCost::constant(1),
            CardResources::mental(),
            CardTraits::single(CardTrait::Armor),
        ))
        .id()
}
