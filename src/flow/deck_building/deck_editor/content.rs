use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::{
    cards::{Aspect, SetTrait},
    component::Card,
    flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState},
    node_ui::{CardDetailButton, CardNode, MouseControl, ScrollingList},
    resource::{AspectCardDatas, AssetLoader},
    util::DeckUtil,
};
pub struct DeckEditorContentPlugin;

impl Plugin for DeckEditorContentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_deck_info_changed.run_if(in_state(DeckBuildingState::DeckEditor)),
        )
        .add_observer(on_content_added);
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
        .entity(trigger.target())
        .insert(Node {
            display: Display::Flex,
            column_gap: Val::Px(5.),
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            overflow: Overflow::scroll_y(),
            ..default()
        })
        .with_children(|parent| {
            let deck_info = DeckInfo::new(res, aspect_card_datas);
            spawn_deck(parent, &deck_info.deck_cards, &asset_loader);
            spawn_info(
                parent,
                deck_info.identity_cards,
                deck_info.current_aspects,
                deck_info.card_count,
                &asset_loader,
            );
            spawn_collection(parent, deck_info.avaiable_cards, &asset_loader);
        });
}

#[derive(Component)]
#[require(RelativeCursorPosition)]
pub enum DeckContent {
    DeckScrollingList,
    CurrentAspects,
    CardCount,
    CollectionScrollingList,
}

#[derive(Component, Clone, PartialEq)]
pub enum CardFrom {
    Deck,
    Collection,
}

fn on_deck_info_changed(
    mut commands: Commands,
    mut content_q: Query<(Entity, &DeckContent, Option<&mut Text>)>,
    res: Res<DeckBuildingResource>,
    aspect_card_datas: Res<AspectCardDatas>,
    asset_loader: Res<AssetLoader>,
) {
    if res.is_changed() {
        if res.get_deck().is_some() {
            let deck_info = DeckInfo::new(res, aspect_card_datas);
            for (entity, content, text_op) in content_q.iter_mut() {
                match content {
                    DeckContent::CurrentAspects => {
                        let current_aspects = deck_info.current_aspects.clone();
                        commands
                            .entity(entity)
                            .despawn_related::<Children>()
                            .with_children(|aspects_container| {
                                if current_aspects.is_empty() {
                                    aspects_container.spawn(Text::new("No Aspect"));
                                } else {
                                    for aspect in current_aspects {
                                        aspects_container.spawn((
                                            Text::new(aspect.to_str()),
                                            TextColor(aspect.get_color().unwrap_or(Color::WHITE)),
                                        ));
                                    }
                                }
                            });
                    }
                    DeckContent::CardCount => {
                        text_op.unwrap().0 = deck_info.card_count.to_string();
                    }
                    DeckContent::DeckScrollingList => {
                        commands
                            .entity(entity)
                            .despawn_related::<Children>()
                            .with_children(|deck_scrolling_list| {
                                spawn_card_list(
                                    deck_scrolling_list,
                                    &deck_info.deck_cards,
                                    &asset_loader,
                                    CardFrom::Deck,
                                );
                            });
                    }
                    DeckContent::CollectionScrollingList => {
                        commands
                            .entity(entity)
                            .despawn_related::<Children>()
                            .with_children(|collection_scrolling_list| {
                                spawn_card_list(
                                    collection_scrolling_list,
                                    &deck_info.avaiable_cards,
                                    &asset_loader,
                                    CardFrom::Collection,
                                );
                            });
                    }
                }
            }
        }
    }
}

fn spawn_deck(
    parent: &mut ChildSpawnerCommands,
    deck_cards: &Vec<Card<'static>>,
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
                    spawn_card_list(scrolling_list, &deck_cards, asset_loader, CardFrom::Deck);
                });
        });
}

fn spawn_info(
    parent: &mut ChildSpawnerCommands,
    identity_cards: Vec<Card<'static>>,
    current_aspects: Vec<Aspect>,
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
                                            CardNode::small(
                                                asset_loader.get(&card.get_key()).clone(),
                                            ),
                                            CardDetailButton,
                                            card.clone(),
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
                        aspect_info
                            .spawn((
                                DeckContent::CurrentAspects,
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                            ))
                            .with_children(|aspects_container| {
                                if current_aspects.is_empty() {
                                    aspects_container.spawn(Text::new("No Aspect"));
                                } else {
                                    for aspect in current_aspects {
                                        aspects_container.spawn((
                                            Text::new(aspect.to_str()),
                                            TextColor(aspect.get_color().unwrap_or(Color::WHITE)),
                                        ));
                                    }
                                }
                            });
                    });

                info_content
                    .spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                    .with_children(|card_count_info| {
                        card_count_info.spawn(Text::new("Card Count:"));
                        card_count_info.spawn((
                            Text::new(selectable_card_count.to_string()),
                            DeckContent::CardCount,
                        ));
                    });
            });
        });
}

fn spawn_collection(
    parent: &mut ChildSpawnerCommands,
    available_cards: Vec<Card<'static>>,
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
                    spawn_card_list(
                        scrolling_list,
                        &available_cards,
                        asset_loader,
                        CardFrom::Collection,
                    );
                });
        });
}

fn spawn_card_list(
    parent: &mut ChildSpawnerCommands,
    cards: &Vec<Card<'static>>,
    asset_loader: &Res<AssetLoader>,
    card_from: CardFrom,
) {
    for card in cards {
        parent
            .spawn(Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_child((
                CardNode::small(asset_loader.get(&card.get_key()).clone()),
                card.clone(),
                MouseControl::default(),
                card_from.clone(),
            ));
    }
}

struct DeckInfo {
    identity_cards: Vec<Card<'static>>,
    deck_cards: Vec<Card<'static>>,
    current_aspects: Vec<Aspect>,
    card_count: usize,
    avaiable_cards: Vec<Card<'static>>,
}

impl DeckInfo {
    pub fn new(res: Res<DeckBuildingResource>, aspect_card_datas: Res<AspectCardDatas>) -> Self {
        let (identity_cards, player_cards) = DeckUtil::get_cards_pair(res.get_identity().unwrap());
        let aspect_card_ids = res.get_deck().unwrap().get_card_ids();
        let aspect_cards = aspect_card_datas.get_batch_card_by_id(&aspect_card_ids);
        let current_aspects = DeckUtil::get_current_aspects(&aspect_cards);
        let deck_card_counts = aspect_cards.len() + 15;
        let avaiable_cards = DeckUtil::get_available_cards(&aspect_card_ids, &current_aspects);
        Self {
            identity_cards,
            current_aspects,
            card_count: deck_card_counts,
            avaiable_cards,
            deck_cards: player_cards
                .iter()
                .chain(aspect_cards.iter())
                .cloned()
                .collect(),
        }
    }
}
