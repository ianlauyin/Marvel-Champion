use bevy::ecs::{entity::Entity, system::Commands};

use crate::{cards::*, component::card::*};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_068",
        name: "Vision",
        sub_name: Some("Vision"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::Aspect(Aspect::Leadership).into(),
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
            PlayerCardType::Ally,
            CardCost::constant(4),
            CardResources::physical(),
            CardTraits::new(vec![CardTrait::Android, CardTrait::Avenger]),
            CardCharacter::ally(3, 1, 1, 2, 1),
        ))
        .id()
}
