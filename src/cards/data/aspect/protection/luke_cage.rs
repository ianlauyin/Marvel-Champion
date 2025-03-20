use bevy::ecs::{entity::Entity, system::Commands};


use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_076",
        name: "Luke Cage",
        sub_name: Some("Luke Cage"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Protection).into(),
    }
}

pub fn get_card() -> (CardBasic<'static>, fn(Commands) -> Entity) {
    (get_info(), spawn_bundle)
}

fn spawn_bundle(mut commands: Commands) -> Entity {
    commands.spawn((
        get_info(),
        PlayerCardType::Ally,
        CardCost::constant(4),
        CardResources::energy(),
        CardKeywords::single(CardKeyword::Toughness),
        CardTraits::single(CardTrait::Defender),
        CardCharacter::ally(5, 1, 1, 2, 1),
    )).id()
}
