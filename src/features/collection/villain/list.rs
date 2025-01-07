use bevy::prelude::*;

use crate::{
    features::{
        cards::Villain,
        collection::state::CollectionState,
        shared::{ListBuilder, ListItem, MenuBuilder},
    },
    systems::{clean_up, LoadAsset},
};

use super::{card_list::CollectionVillainSet, state::CollectionVillainState};

pub struct CollectionVillainListPlugin;

impl Plugin for CollectionVillainListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CollectionState::Villain), spawn_villain_list)
            .add_systems(
                Update,
                handle_button_interaction.run_if(in_state(CollectionVillainState::List)),
            )
            .add_systems(OnExit(CollectionState::Villain), clean_up::<VillainList>);
    }
}

#[derive(Component, Clone)]
struct VillainList;

fn spawn_villain_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let villains = Villain::get_all();
    let list_map = villains
        .iter()
        .map(|villain| {
            (
                villain.clone(),
                ListItem {
                    text: villain.to_string().clone(),
                    image: ImageNode::new(asset_server.load(villain.get_title_image_path()))
                        .with_color(Color::srgb(0.365, 0.365, 0.365)),
                    ..default()
                },
            )
        })
        .collect();
    let content_child = ListBuilder(list_map).spawn(commands.reborrow());
    MenuBuilder {
        next_state: None::<CollectionState>,
        component: VillainList,
        previous_state: CollectionState::Menu,
        content_child,
    }
    .spawn(commands);
}

fn handle_button_interaction(
    mut commands: Commands,
    button_q: Query<(&Interaction, &Villain), With<Button>>,
    mut next_state: ResMut<NextState<CollectionVillainState>>,
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, villain) in button_q.iter() {
        if *interaction == Interaction::Pressed {
            for card in villain.get_cards() {
                load_asset.add_card(&card, &asset_server);
            }
            commands.insert_resource(CollectionVillainSet(villain.clone()));
            next_state.set(CollectionVillainState::LoadingCards);
            return;
        }
    }
}
