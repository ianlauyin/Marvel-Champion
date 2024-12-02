use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, ModularSet},
        shared::{
            handle_previous_interaction, spawn_card_detail, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
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
                Update,
                handle_previous_interaction(CollectionModularState::Cards),
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

#[derive(Component, Clone)]
struct ModularCardButton(Card);

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
                ModularCardButton(card.clone()),
                ListItem {
                    image: UiImage::new(asset_server.load(card.get_card_image_path())),
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
    modular_card_button_q: Query<(&Interaction, &ModularCardButton, &ZIndex)>,
) {
    for (interaction, modular_card_button, z_index) in modular_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = match z_index {
                ZIndex::Local(value) => ZIndex::Local(value + 1),
                ZIndex::Global(value) => ZIndex::Global(value + 1),
            };
            spawn_card_detail(
                commands,
                asset_server,
                modular_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
