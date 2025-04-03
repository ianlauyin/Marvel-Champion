use bevy::prelude::*;
use serde::de;

use crate::{
    cards::Aspect,
    component::card::CardBasic,
    flow::deck_building::resource::DeckBuildingResource,
    resource::{AspectCardDatas, AssetLoader},
    ui_component::{Card, ScrollingList},
    util::DeckUtil,
};
pub struct DeckEditorContentPlugin;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_content_added);
    }
}

#[derive(Component)]
pub struct DeckEditorContent;

fn on_content_added(
    trigger: Trigger<OnAdd, DeckEditorContent>,
    mut commands: Commands,
    res: Res<DeckBuildingResource>,
    aspect_card_datas: Res<AspectCardDatas>,
    asset_loader: Res<AssetLoader>,
) {
    commands
        .entity(trigger.entity())
        .insert(Node {
            display: Display::Flex,
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            overflow: Overflow::scroll_y(),
            ..default()
        })
        .with_children(|parent| {
            let (identity_cards, other_cards) =
                DeckUtil::get_cards_pair(res.get_identity().unwrap());
            let aspect_card_ids = res.get_deck().unwrap().get_card_ids();
            let aspect_cards = aspect_card_datas.get_batch_info_by_id(&aspect_card_ids);
            spawn_deck(parent, &other_cards, &aspect_cards, &asset_loader);

            let current_aspect = DeckUtil::get_current_aspect(&aspect_cards);
            let deck_card_counts = aspect_cards.len() + other_cards.len();
            spawn_info(
                parent,
                &identity_cards,
                current_aspect,
                deck_card_counts,
                &asset_loader,
            );

            let avaiable_cards = DeckUtil::get_available_cards(&aspect_card_ids);
            spawn_collection(parent, avaiable_cards, &asset_loader);
        });
}

#[derive(Component)]
enum DeckContent {
    Deck,
    Info,
    Collection,
}

fn spawn_deck(
    parent: &mut ChildBuilder,
    player_cards: &Vec<CardBasic>,
    aspect_cards: &Vec<CardBasic>,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn((
            DeckContent::Deck,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(45.),
                row_gap: Val::Px(10.),
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ))
        .with_children(|current| {
            current.spawn(Text::new("Deck:"));
            current
                .spawn((
                    ScrollingList::grid(7, 10.),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(5.)),
                ))
                .with_children(|scrolling_list| {
                    for card in player_cards.iter().chain(aspect_cards.iter()) {
                        scrolling_list
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_child(Card::small(asset_loader.get(&card.get_key()).clone()));
                    }
                });
        });
}

fn spawn_info(
    parent: &mut ChildBuilder,
    identity_cards: &Vec<CardBasic>,
    current_aspect: Option<Aspect>,
    selectable_card_count: usize,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn((
            DeckContent::Info,
            Node {
                width: Val::Percent(10.),
                ..default()
            },
        ))
        .with_children(|info| {
            info.spawn(Text::new("Info:"));
        });
}

fn spawn_collection(
    parent: &mut ChildBuilder,
    available_cards: Vec<CardBasic>,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn((
            DeckContent::Collection,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(45.),
                row_gap: Val::Px(10.),
                overflow: Overflow::scroll_y(),
                ..default()
            },
        ))
        .with_children(|available| {
            available.spawn(Text::new("Collection:"));
            available
                .spawn((
                    ScrollingList::grid(7, 10.),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(5.)),
                ))
                .with_children(|scrolling_list| {
                    for card in available_cards {
                        scrolling_list
                            .spawn(Node {
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_child(Card::small(asset_loader.get(&card.get_key()).clone()));
                    }
                });
        });
}
