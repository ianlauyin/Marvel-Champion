use bevy::prelude::*;

use crate::features::{
    cards::Villain,
    shared::{ListBuilder, ListItem, MenuBuilder},
};

use super::state::GameSelectorState;

pub struct GameSelectorEncounterPlugin;

const CURRENT_STATE: GameSelectorState = GameSelectorState::Encounter;

impl Plugin for GameSelectorEncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_encounter_list);
    }
}

#[derive(Component, Clone)]
struct VillainList;

#[derive(Component, Clone)]
struct VillainItem(Villain);

fn spawn_encounter_list(mut commands: Commands, asset_server: Res<AssetServer>) {
    let villain = Villain::get_all();
    let list_map = villain
        .iter()
        .map(|villain| {
            (
                VillainItem(villain.clone()),
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
        component: VillainList,
        previous_state: GameSelectorState::Identity,
        content_child,
    }
    .spawn(commands);
}
