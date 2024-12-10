use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, CardDatas},
        collection::state::CollectionState,
                shared::{
            handle_previous_interaction, CardDetailBuilder, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
    utils::get_largest_z_index,
};

use super::state::CollectionBasicState;

pub struct CollectionBasicCardListPlugin;

impl Plugin for CollectionBasicCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionBasicState::Cards), spawn_basic_cards)
            .add_systems(
                Update,
                handle_card_click
                    .run_if(in_state(CollectionBasicState::Cards))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(Update, handle_previous_interaction(CollectionState::Basic))
            .add_systems(
                OnExit(CollectionBasicState::Cards),
                clean_up::<BasicCardList>,
            );
    }
}

#[derive(Component, Clone)]
struct BasicCardList;

fn spawn_basic_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_basic_cards()
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
        component: BasicCardList,
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
            CardDetailBuilder {
                card: card.clone(),
                position: Vec2::ZERO,
                z_index: get_largest_z_index(z_index_q),
            }
            .spawn(commands, asset_server);
            return;
        }
    }
}
