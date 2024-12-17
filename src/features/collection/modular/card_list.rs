use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, ModularSet},
        shared::{CardDetailBuilder, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::clean_up,
    utils::get_largest_z_index,
};

use super::state::CollectionModularState;

pub struct CollectionModularCardListPlugin;

impl Plugin for CollectionModularCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionModularState::Cards), spawn_modular_cards)
            .add_systems(
                Update,
                handle_card_click
                    .run_if(in_state(CollectionModularState::Cards))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(
                OnExit(CollectionModularState::Cards),
                clean_up::<ModularCardList>,
            );
    }
}

#[derive(Resource)]
pub struct CollectionModularSet(pub ModularSet);

#[derive(Component, Clone)]
struct ModularCardList;

fn spawn_modular_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionModularSet>,
) {
    let list_items = identity
        .0
        .get_cards()
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
        component: ModularCardList,
        previous_state: CollectionModularState::List,
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
