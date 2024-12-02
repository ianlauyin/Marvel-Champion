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

use super::state::CollectionLeadershipState;

pub struct CollectionLeadershipCardListPlugin;

impl Plugin for CollectionLeadershipCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionLeadershipState::Cards),
            spawn_leadership_cards,
        )
        .add_systems(
            Update,
            handle_card_click
                .run_if(in_state(CollectionLeadershipState::Cards))
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            handle_previous_interaction(CollectionState::Leadership),
        )
        .add_systems(
            OnExit(CollectionLeadershipState::Cards),
            clean_up::<LeadershipCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct LeadershipCardList;

#[derive(Component, Clone)]
struct LeadershipCardButton(Card);

fn spawn_leadership_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_leadership_cards()
        .iter()
        .map(|card| {
            (
                LeadershipCardButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_card_image_path())),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: LeadershipCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}

fn handle_card_click(
    commands: Commands,
    asset_server: Res<AssetServer>,
    leadership_card_button_q: Query<(&Interaction, &LeadershipCardButton, &ZIndex)>,
) {
    for (interaction, leadership_card_button, z_index) in leadership_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(z_index.0 + 1);
            spawn_card_detail(
                commands,
                asset_server,
                leadership_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
