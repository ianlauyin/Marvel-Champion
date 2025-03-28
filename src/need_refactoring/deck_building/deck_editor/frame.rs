use crate::{
    features::{cards::CardDatas, deck_building::deck_list::EditIdentity},
    systems::{clean_up, LoadedAssetMap, StorageDeck},
};
use bevy::prelude::*;

use super::{super::state::DeckBuildingState, content::spawn_content, header::spawn_header};

#[derive(Resource, Clone)]
pub struct EditingDeck {
    pub index: Option<usize>,
    pub deck: StorageDeck,
}

impl EditingDeck {
    pub fn new() -> Self {
        Self {
            index: None,
            deck: StorageDeck {
                name: "New Deck".to_string(),
                card_ids: vec![],
            },
        }
    }
}

pub struct DeckEditorFramePlugin;

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;

impl Plugin for DeckEditorFramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_editor)
            .add_systems(OnExit(CURRENT_STATE), clean_up::<DeckEditor>);
    }
}

#[derive(Component)]
struct DeckEditor;

pub fn spawn_editor(
    mut commands: Commands,
    editing_deck: Res<EditingDeck>,
    edit_identity: Res<EditIdentity>,
    card_datas: Res<CardDatas>,
    loaded_asset_map: Res<LoadedAssetMap>,
) {
    let card_list_items = card_datas.from_ids(&editing_deck.deck.card_ids);
    let header = spawn_header(commands.reborrow(), editing_deck.deck.name.clone());
    let content = spawn_content(
        commands.reborrow(),
        &edit_identity.0,
        card_list_items,
        loaded_asset_map,
    );
    commands
        .spawn((
            DeckEditor,
            Node {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ))
        .add_children(&[header, content]);
}
