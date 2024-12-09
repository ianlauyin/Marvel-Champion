use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, Villain},
        shared::{
            handle_previous_interaction, spawn_card_detail, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
    utils::get_largest_z_index,
};

use super::state::CollectionVillainState;

pub struct CollectionVillainCardListPlugin;

impl Plugin for CollectionVillainCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionVillainState::Cards), spawn_villain_cards)
            .add_systems(
                Update,
                handle_card_click
                    .run_if(in_state(CollectionVillainState::Cards))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(
                Update,
                handle_previous_interaction(CollectionVillainState::Cards),
            )
            .add_systems(
                OnExit(CollectionVillainState::Cards),
                clean_up::<VillainCardList>,
            );
    }
}

#[derive(Resource)]
pub struct CollectionVillainSet(pub Villain);

#[derive(Component, Clone)]
struct VillainCardList;

fn spawn_villain_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionVillainSet>,
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
        component: VillainCardList,
        previous_state: CollectionVillainState::List,
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
