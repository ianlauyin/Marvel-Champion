use bevy::prelude::*;

use crate::{
    features::{
        cards::ModularSet,
        collection::state::CollectionState,
        shared::{handle_previous_interaction, DisplayMethod, ListItem, MenuBuilder},
    },
    systems::{clean_up, LoadAsset},
};

use super::{card_list::CollectionModularSet, state::CollectionModularState};

pub struct CollectionModularListPlugin;

impl Plugin for CollectionModularListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Modular), spawn_modular_list)
            .add_systems(
                Update,
                handle_previous_interaction(CollectionState::Modular),
            )
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CollectionModularState::List)),
            )
            .add_systems(OnExit(CollectionState::Modular), clean_up::<ModularList>);
    }
}

#[derive(Component, Clone)]
struct ModularList;

fn spawn_modular_list(commands: Commands, asset_server: Res<AssetServer>) {
    let modular_sets = ModularSet::get_all();
    let button_map = modular_sets
        .iter()
        .map(|modular_set| {
            (
                modular_set.clone(),
                ListItem {
                    text: modular_set.to_string().clone(),
                    image: ImageNode::new(asset_server.load(modular_set.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    MenuBuilder {
        component: ModularList,
        previous_state: CollectionState::Menu,
        list_items: button_map,
        display_method: DisplayMethod::ButtonList,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &ModularSet), With<Button>>,
    mut next_state: ResMut<NextState<CollectionModularState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, modular_set) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in modular_set.get_cards() {
                load_asset.add_card(card, &asset_server);
            }
            commands.insert_resource(CollectionModularSet(modular_set.clone()));
            next_state.set(CollectionModularState::LoadingCards);
            return;
        }
    }
}
