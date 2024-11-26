use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    features::{
        cards::{Card, CardDatas, Identity},
        shared::{
            handle_previous_interaction, spawn_card_detail, DisplayMethod, ListItem, MenuBuilder,
        },
    },
    systems::clean_up,
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

#[derive(Component, Clone)]
struct HeroCardButton(Card);

fn spawn_hero_cards(
    commands: Commands,
    asset_server: Res<AssetServer>,
    identity: Res<CollectionHeroIdentity>,
) {
    let list_items = CardDatas::get_identity_cards(identity.0.clone())
        .iter()
        .map(|card| {
            (
                HeroCardButton(card.clone()),
                ListItem {
                    image: UiImage::new(asset_server.load(card.get_card_image_path())),
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
    hero_card_button_q: Query<(&Interaction, &HeroCardButton, &ZIndex)>,
) {
    for (interaction, hero_card_button, z_index) in hero_card_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let card_detail_z_index = match z_index {
                ZIndex::Local(value) => ZIndex::Local(value + 1),
                ZIndex::Global(value) => ZIndex::Global(value + 1),
            };
            spawn_card_detail(
                commands,
                asset_server,
                hero_card_button.0.clone(),
                Vec2::ZERO,
                card_detail_z_index,
            );
            return;
        }
    }
}
