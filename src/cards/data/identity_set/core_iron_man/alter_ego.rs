use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_029b",
        name: "Tony Stark",
        sub_name: Some("Iron Man"),
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
            IdentityCardType::AlterEgo {
                flip_target_id: vec!["core_029a"],
                hand_size: 6,
            },
            CardCharacter::alter_ego(9, 3),
            CardTraits::single(CardTrait::Genius),
        ))
        .id()
}
