use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, CardDatas},
        collection::state::CollectionState,
        shared::{
            handle_previous_interaction, spawn_card_detail, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
};

use super::state::CollectionProtectionState;

pub struct CollectionProtectionCardListPlugin;

impl Plugin for CollectionProtectionCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionProtectionState::Cards),
            spawn_protection_cards,
        )
        .add_systems(
            Update,
            handle_card_click
                .run_if(in_state(CollectionProtectionState::Cards))
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            handle_previous_interaction(CollectionState::Protection),
        )
        .add_systems(
            OnExit(CollectionProtectionState::Cards),
            clean_up::<ProtectionCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct ProtectionCardList;

fn spawn_protection_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_protection_cards()
        .iter()
        .map(|card| {
            (
                card.clone(),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_image_path())),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: ProtectionCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}

fn handle_card_click(
    commands: Commands,
    asset_server: Res<AssetServer>,
    card_button_q: Query<(&Interaction, &Card), With<Button>>,
    z_index_q: Query<&ZIndex>,
) {
    let current_largest_z_index_i32 = z_index_q.iter().map(|z_index| z_index.0).max().unwrap();
    for (interaction, card) in card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(current_largest_z_index_i32 + 1);
            spawn_card_detail(
                commands,
                asset_server,
                card.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
