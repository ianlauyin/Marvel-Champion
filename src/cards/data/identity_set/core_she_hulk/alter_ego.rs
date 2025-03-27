use crate::{cards::*, component::card::*};
use bevy::ecs::{entity::Entity, system::Commands};

pub fn get_info() -> CardBasic<'static> {
    CardBasic {
        id: "core_019b",
        name: "Jennifer Walters",
        sub_name: Some("She-Hulk"),
        unique: true,
        card_amount_max: 1,
        belongs: Belong::IdentitySet(IdentitySet::CoreSheHulk).into(),
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
                flip_target_id: vec!["core_019a"],
                hand_size: 6,
            },
            CardTraits::new(vec![CardTrait::Attorney, CardTrait::Gamma]),
            CardCharacter::alter_ego(15, 5),
        ))
        .id()
}

// pub fn get_alter_ego() -> Card {
//     Card::AlterEgo(AlterEgoCard {
//         id: "core_019b",
//         name: "Jennifer Walters",
//         aspect: CardAspect::IdentitySpecific(CoreSheHulk),
//         card_icons: vec![],
//         description: "Interrupt: When threat would be placed on a scheme, prevent 1 of that threat. (Limit once per round.)",
//         abilities: vec![],
//         card_image_path: "embedded://cards/identity/core_she_hulk/core_019b.png",
//         card_back_image_path: "embedded://cards/identity/core_she_hulk/core_019a.png",
//         flip_target_id: vec!["core_019a"],
//         initial_hit_points: 15,
//         keywords: vec![],
//         traits: vec![CardTrait::Attorney, CardTrait::Gamma],
//         hand_size: 6,
//         nemesis_id: "core_162",
//         nemesis_side_scheme_id: "core_161",
//         nemesis_card_id: vec!["core_163", "core_164"],
//         rec: 5,
//     })
// }
