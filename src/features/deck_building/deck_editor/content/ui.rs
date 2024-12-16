use bevy::prelude::*;

use crate::{
    constants::CARD_SIZE,
    features::{
        cards::{Card, CardDatas},
        deck_building::{deck_editor::EditingDeck, state::DeckBuildingState},
        shared::CardListBuilder,
    },
    systems::LoadedAssetMap,
    utils::get_card_amount,
};

use super::utils::{convert_card_into_button_map, get_aspect_names};

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckBuilding;
pub struct DeckEditorContentUIPlugin;

impl Plugin for DeckEditorContentUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_editing_deck_changed.run_if(in_state(CURRENT_STATE)),
        );
    }
}

pub fn spawn_content(
    mut commands: Commands,
    deck_cards: Vec<Card>,
    loaded_asset: Res<LoadedAssetMap>,
) -> Entity {
    let aspect_names = get_aspect_names(&deck_cards);
    let deck_card_list = spawn_deck_card_list(commands.reborrow(), &deck_cards, &loaded_asset);
    let info = spawn_info(
        commands.reborrow(),
        get_card_amount(&deck_cards),
        aspect_names,
    );
    let selection_list = spawn_selection_list(commands.reborrow(), &loaded_asset);

    commands
        .spawn(Node {
            display: Display::Flex,
            height: Val::Percent(100.),
            ..default()
        })
        .add_children(&[deck_card_list, info, selection_list])
        .id()
}

#[derive(Component, Clone, PartialEq, Eq)]
pub enum ContentContainer {
    Deck,
    Selection,
}

#[derive(Component, Clone)]
pub enum CardListItem {
    Deck,
    Selection,
}

#[derive(Component, Clone)]
enum DeckInfo {
    Amount,
    Aspects,
}

fn spawn_deck_card_list(
    mut commands: Commands,
    deck_cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Entity {
    let list_items = convert_card_into_button_map(CardListItem::Deck, deck_cards, &loaded_asset);
    let list = CardListBuilder {
        button_map: list_items.clone(),
        card_size: (
            Val::Px(CARD_SIZE.truncate().x),
            Val::Px(CARD_SIZE.truncate().y),
        ),
        height: Val::Percent(90.),
        columns: 7,
    }
    .spawn(commands.reborrow());
    commands
        .spawn((
            ContentContainer::Deck,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
        .add_child(list)
        .id()
}

fn spawn_info(
    mut commands: Commands,
    deck_cards_amount: u8,
    aspects: Vec<(String, Color)>,
) -> Entity {
    commands
        .spawn(Node {
            width: Val::Percent(10.),
            padding: UiRect::vertical(Val::Px(30.)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|info_container| {
            info_container
                .spawn((
                    Text::new(format!("Cards: ")),
                    Node {
                        margin: UiRect::bottom(Val::Px(50.)),
                        ..default()
                    },
                ))
                .with_child((
                    TextSpan::new(deck_cards_amount.to_string()),
                    DeckInfo::Amount,
                ));
            info_container
                .spawn((
                    Text::new("Aspects: "),
                    TextLayout::new_with_linebreak(LineBreak::WordBoundary),
                    DeckInfo::Aspects,
                ))
                .with_children(|aspect_node| {
                    for (aspect, color) in aspects {
                        aspect_node.spawn((
                            TextSpan::new(format!("{} ", aspect)),
                            TextColor(color),
                            TextFont::from_font_size(16.),
                        ));
                    }
                });
        })
        .id()
}

fn spawn_selection_list(mut commands: Commands, loaded_asset: &Res<LoadedAssetMap>) -> Entity {
    let cards = CardDatas::get_aspect_cards();
    let list_items = convert_card_into_button_map(CardListItem::Selection, &cards, loaded_asset);
    let list = CardListBuilder {
        button_map: list_items,
        card_size: (
            Val::Px(CARD_SIZE.truncate().x),
            Val::Px(CARD_SIZE.truncate().y),
        ),
        height: Val::Percent(90.),
        columns: 7,
    }
    .spawn(commands.reborrow());

    commands
        .spawn((
            ContentContainer::Selection,
            Node {
                width: Val::Percent(45.),
                ..default()
            },
        ))
        .add_child(list)
        .id()
}

fn handle_editing_deck_changed(
    mut commands: Commands,
    editing_deck: Res<EditingDeck>,
    content_container_q: Query<(Entity, &ContentContainer)>,
    loaded_asset: Res<LoadedAssetMap>,
    card_datas: Res<CardDatas>,
) {
    if editing_deck.is_changed() {
        let cards = card_datas.from_ids(&editing_deck.deck.card_ids);
        for (entity, content_container) in content_container_q.iter() {
            match *content_container {
                ContentContainer::Deck => handle_deck_container_change(
                    commands.reborrow(),
                    entity,
                    cards.clone(),
                    &loaded_asset,
                ),
                _ => {}
            }
        }
    }
}

fn handle_deck_container_change(
    mut commands: Commands,
    container: Entity,
    cards: Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) {
    let deck_list = CardListBuilder {
        button_map: convert_card_into_button_map(CardListItem::Deck, &cards, loaded_asset),
        card_size: (
            Val::Px(CARD_SIZE.truncate().x),
            Val::Px(CARD_SIZE.truncate().y),
        ),
        height: Val::Percent(90.),
        columns: 7,
    }
    .spawn(commands.reborrow());
    let mut deck_list_container = commands.entity(container);

    deck_list_container.despawn_descendants();
    deck_list_container.add_child(deck_list);
    return;
}
