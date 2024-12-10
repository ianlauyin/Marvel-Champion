use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, Identity},
                shared::{
            handle_previous_interaction, CardDetailBuilder, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
    utils::get_largest_z_index,
};

use super::state::CollectionHeroState;

pub struct CollectionHeroCardListPlugin;

impl Plugin for CollectionHeroCardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionHeroState::Cards), spawn_hero_cards)
            .add_systems(
                Update,
                handle_card_click
                    .run_if(in_state(CollectionHeroState::Cards))
                    .run_if(input_just_pressed(MouseButton::Left)),
            )
            .add_systems(
                Update,
                handle_previous_interaction(CollectionHeroState::Cards),
            )
            .add_systems(OnExit(CollectionHeroState::Cards), clean_up::<HeroCardList>);
    }
}

#[derive(Resource)]
pub struct CollectionHeroIdentity(pub Identity);

#[derive(Component, Clone)]
struct HeroCardList;

fn spawn_hero_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionHeroIdentity>,
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
        component: HeroCardList,
        previous_state: CollectionHeroState::List,
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
