use bevy::prelude::*;

use crate::{
    cards::{Aspect, SetTrait},
    component::card::CardBasic,
    flow::deck_building::resource::DeckBuildingResource,
    resource::{AspectCardDatas, AssetLoader},
    ui_component::{Card, CardDetailButton, ScrollingList},
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
            column_gap: Val::Px(5.),
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
                identity_cards,
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
    DeckScrollingList,
    CurrentAspect,
    CardCount,
    CollectionScrollingList,
}

fn spawn_deck(
    parent: &mut ChildBuilder,
    player_cards: &Vec<CardBasic>,
    aspect_cards: &Vec<CardBasic>,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(40.),
            row_gap: Val::Px(10.),
            overflow: Overflow::scroll_y(),
            ..default()
        })
        .with_children(|current| {
            current
                .spawn((
                    BorderColor(Color::WHITE),
                    Node {
                        border: UiRect::bottom(Val::Px(1.)),
                        ..default()
                    },
                ))
                .with_child(Text::new("Deck:"));
            current
                .spawn((ScrollingList::grid(6, 10.), DeckContent::DeckScrollingList))
                .with_children(|scrolling_list| {
                    let cards: Vec<CardBasic> = player_cards
                        .iter()
                        .chain(aspect_cards.iter())
                        .cloned()
                        .collect();
                    spawn_card_list(scrolling_list, cards, asset_loader);
                });
        });
}

fn spawn_info(
    parent: &mut ChildBuilder,
    identity_cards: Vec<CardBasic>,
    current_aspect: Option<Aspect>,
    selectable_card_count: usize,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn((Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(20.),
            padding: UiRect::horizontal(Val::Px(5.)),
            ..default()
        },))
        .with_children(|info| {
            info.spawn((
                BorderColor(Color::WHITE),
                Node {
                    width: Val::Percent(100.),
                    border: UiRect::bottom(Val::Px(1.)),
                    ..default()
                },
            ))
            .with_child(Text::new("Info:"));

            info.spawn(Node {
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            })
            .with_children(|info_content| {
                info_content
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|identity_info| {
                        identity_info.spawn(Text::new("Identity Cards:"));
                        identity_info
                            .spawn(Node {
                                display: Display::Flex,
                                column_gap: Val::Px(5.),
                                ..default()
                            })
                            .with_children(|identity_card_container| {
                                for card in identity_cards {
                                    identity_card_container
                                        .spawn(Node {
                                            display: Display::Flex,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        })
                                        .with_child((
                                            Card::small(asset_loader.get(&card.get_key()).clone()),
                                            CardDetailButton::new(card.get_key(), card.is_vertical),
                                        ));
                                }
                            });
                    });

                info_content
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|aspect_info| {
                        aspect_info.spawn(Text::new("Current Aspect:"));
                        let (aspect_name, color) = get_aspect_name_and_color(current_aspect);
                        aspect_info.spawn((
                            Text::new(aspect_name),
                            TextColor(color),
                            DeckContent::CurrentAspect,
                        ));
                    });

                info_content
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|card_count_info| {
                        card_count_info.spawn(Text::new("Selectable Card Count:"));
                        card_count_info.spawn((
                            Text::new(selectable_card_count.to_string()),
                            DeckContent::CardCount,
                        ));
                    });
            });
        });
}

fn spawn_collection(
    parent: &mut ChildBuilder,
    available_cards: Vec<CardBasic>,
    asset_loader: &Res<AssetLoader>,
) {
    parent
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            width: Val::Percent(40.),
            row_gap: Val::Px(10.),
            overflow: Overflow::scroll_y(),
            ..default()
        })
        .with_children(|available| {
            available
                .spawn((
                    BorderColor(Color::WHITE),
                    Node {
                        border: UiRect::bottom(Val::Px(1.)),
                        ..default()
                    },
                ))
                .with_child(Text::new("Collection:"));
            available
                .spawn((
                    ScrollingList::grid(6, 10.),
                    DeckContent::CollectionScrollingList,
                ))
                .with_children(|scrolling_list| {
                    spawn_card_list(scrolling_list, available_cards, asset_loader);
                });
        });
}

fn spawn_card_list(
    parent: &mut ChildBuilder,
    cards: Vec<CardBasic>,
    asset_loader: &Res<AssetLoader>,
) {
    for card in cards {
        parent
            .spawn(Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_child((Card::small(asset_loader.get(&card.get_key()).clone()),));
    }
}

fn get_aspect_name_and_color(aspect: Option<Aspect>) -> (String, Color) {
    if let Some(aspect) = aspect {
        (
            aspect.to_str().to_string(),
            aspect.get_color().unwrap_or(Color::WHITE),
        )
    } else {
        ("No Aspect".to_string(), Color::WHITE)
    }
}
