use bevy::prelude::*;

use crate::{
    constants::{ENCOUNTER_CARD_BACK_ASSET, PLAYER_CARD_BACK_ASSET, VILLAIN_CARD_BACK_ASSET},
    features::{
        cards::CardDatas,
        game::game_selector::{SelectedEncounterSet, SelectedPlayers},
    },
    systems::LoadAsset,
    ui::GamePlayCamera,
};

use super::{
    game_elements::{EnemyOperation, PlayerOperation},
    state::InGameState,
};

pub struct InGameSetupPlugin;

impl Plugin for InGameSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::LoadingCards), add_loading_cards)
            .add_systems(OnEnter(InGameState::Setup), setup_game);
    }
}

fn add_loading_cards(
    mut load_asset: ResMut<LoadAsset>,
    asset_server: Res<AssetServer>,
    card_datas: Res<CardDatas>,
    selected_players: Res<SelectedPlayers>,
    selected_encounter_set: Res<SelectedEncounterSet>,
) {
    load_asset.0.append(&mut vec![
        ENCOUNTER_CARD_BACK_ASSET.into_load_asset(&asset_server),
        PLAYER_CARD_BACK_ASSET.into_load_asset(&asset_server),
        VILLAIN_CARD_BACK_ASSET.into_load_asset(&asset_server),
    ]);
    for player in selected_players.0.iter() {
        for card_id in player.deck.card_ids.iter() {
            load_asset.add_card_by_id(card_id, &asset_server, &card_datas);
        }
    }
    for villain_card in selected_encounter_set.villain.get_cards().iter() {
        load_asset.add_card(villain_card, &asset_server);
    }
    for modular_set in selected_encounter_set.modular_sets.iter() {
        for modular_card in modular_set.get_cards().iter() {
            load_asset.add_card(modular_card, &asset_server);
        }
    }
}

fn setup_game(
    mut commands: Commands,
    mut game_play_camera: ResMut<GamePlayCamera>,
    card_datas: Res<CardDatas>,
    selected_players: Res<SelectedPlayers>,
    selected_encounter_set: Res<SelectedEncounterSet>,
) {
    game_play_camera.toggle();
    PlayerOperation::init_game(commands.reborrow(), &selected_players, card_datas);
    EnemyOperation::init_game(commands, &selected_players, &selected_encounter_set);
}
