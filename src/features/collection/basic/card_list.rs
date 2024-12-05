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
    basic_card_button_q: Query<(&Interaction, &Card, &ZIndex), With<Button>>,
) {
    for (interaction, card, z_index) in basic_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(z_index.0 + 1);
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
