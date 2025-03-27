use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_040b",
        name: "T'Challa",
        sub_name: Some("Black Panther"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreBlackPanther).into(),
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
            IdentityCardType::AlterEgo {
                hand_size: 6,
                flip_target_id: vec!["core_040a"],
            },
            CardCharacter::alter_ego(11, 4),
            CardTraits::new(vec![CardTrait::King, CardTrait::Wakanda]),
        ))
        .id()
}
