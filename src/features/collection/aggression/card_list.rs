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
    utils::get_largest_z_index,
};

use super::state::CollectionAggressionState;

pub struct CollectionAggressionCardListPlugin;

impl Plugin for CollectionAggressionCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(CollectionAggressionState::Cards),
            spawn_aggression_cards,
        )
        .add_systems(
            Update,
            handle_card_click
                .run_if(in_state(CollectionAggressionState::Cards))
                .run_if(input_just_pressed(MouseButton::Left)),
        )
        .add_systems(
            Update,
            handle_previous_interaction(CollectionState::Aggression),
        )
        .add_systems(
            OnExit(CollectionAggressionState::Cards),
            clean_up::<AggressionCardList>,
        );
    }
}

#[derive(Component, Clone)]
struct AggressionCardList;

fn spawn_aggression_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_aggression_cards()
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
        component: AggressionCardList,
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
    for (interaction, card) in card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            spawn_card_detail(
                commands,
                asset_server,
                card.clone(),
                Vec2::ZERO,
                get_largest_z_index(z_index_q),
            );
            return;
        }
    }
}
