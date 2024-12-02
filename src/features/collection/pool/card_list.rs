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

use super::state::CollectionPoolState;

pub struct CollectionPoolCardListPlugin;

impl Plugin for CollectionPoolCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionPoolState::Cards), spawn_pool_cards)
            .add_systems(
                Update,
                handle_card_click
                    .run_if(in_state(CollectionPoolState::Cards))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(Update, handle_previous_interaction(CollectionState::Pool))
            .add_systems(OnExit(CollectionPoolState::Cards), clean_up::<PoolCardList>);
    }
}

#[derive(Component, Clone)]
struct PoolCardList;

#[derive(Component, Clone)]
struct PoolCardButton(Card);

fn spawn_pool_cards(commands: Commands, asset_server: Res<AssetServer>) {
    let list_items = CardDatas::get_pool_cards()
        .iter()
        .map(|card| {
            (
                PoolCardButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_card_image_path())),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: PoolCardList,
        previous_state: CollectionState::Menu,
        list_items,
        display_method: DisplayMethod::CardList,
    }
    .spawn(commands);
}

fn handle_card_click(
    commands: Commands,
    asset_server: Res<AssetServer>,
    pool_card_button_q: Query<(&Interaction, &PoolCardButton, &ZIndex)>,
) {
    for (interaction, pool_card_button, z_index) in pool_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(z_index.0 + 1);
            spawn_card_detail(
                commands,
                asset_server,
                pool_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
