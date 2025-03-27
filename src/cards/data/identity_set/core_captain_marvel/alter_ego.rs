use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_010b",
        name: "Carol Danvers",
        sub_name: Some("Captain Marvel"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreCaptainMarvel).into(),
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
                flip_target_id: vec!["core_010a"],
                hand_size: 6,
            },
            CardCharacter::alter_ego(12, 4),
            CardTraits::new(vec![CardTrait::SHIELD, CardTrait::Soldier]),
        ))
        .id()
}
