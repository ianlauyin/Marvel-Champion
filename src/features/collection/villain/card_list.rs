use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, VillainSet},
        shared::{
            handle_previous_interaction, spawn_card_detail, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
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
pub struct CollectionVillainSet(pub VillainSet);

#[derive(Component, Clone)]
struct VillainCardList;

#[derive(Component, Clone)]
struct VillainCardButton(Card);

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
                VillainCardButton(card.clone()),
                ListItem {
                    image: ImageNode::new(asset_server.load(card.get_card_image_path())),
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
    villain_card_button_q: Query<(&Interaction, &VillainCardButton, &ZIndex)>,
) {
    for (interaction, villain_card_button, z_index) in villain_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = ZIndex(z_index.0 + 1);
            spawn_card_detail(
                commands,
                asset_server,
                villain_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
