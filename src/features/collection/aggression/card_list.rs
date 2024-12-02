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

#[derive(Component, Clone)]
struct AggressionCardButton(Card);

fn spawn_aggression_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_aggression_cards()
        .iter()
        .map(|card| {
            (
                AggressionCardButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_card_image_path())),
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
    aggression_card_button_q: Query<(&Interaction, &AggressionCardButton, &ZIndex)>,
) {
    for (interaction, aggression_card_button, z_index) in aggression_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(z_index.0 + 1);
            spawn_card_detail(
                commands,
                asset_server,
                aggression_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
