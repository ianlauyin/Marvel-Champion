use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::{
    component::Card,
    constant::CARD_SIZE_SMALL,
    flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState},
    node_ui::{CardDetail, CardNode, MouseControl, MouseControlEvent, ScrollingList},
    resource::{AspectCardDatas, AssetLoader},
    util::DeckUtil,
};

use super::dragging_card::DraggingCard;

pub struct DeckEditorCardListContainerPlugin;

impl Plugin for DeckEditorCardListContainerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_deck_info_changed, handle_mouse_event)
                .run_if(in_state(DeckBuildingState::DeckEditor)),
        )
        .add_observer(on_added);
    }
}

#[derive(Component, Clone, PartialEq)]
pub enum CardFrom {
    Deck,
    Collection,
}

#[derive(Component)]
pub enum CardListContainer {
    Deck,
    Collection,
}

impl CardListContainer {
    fn card_list(&self) -> CardList {
        match self {
            CardListContainer::Deck => CardList::Deck,
            CardListContainer::Collection => CardList::Collection,
        }
    }
}

fn on_added(
    trigger: Trigger<OnAdd, CardListContainer>,
    mut commands: Commands,
    card_list_q: Query<&CardListContainer>,
    res: Res<DeckBuildingResource>,
    asset_loader: Res<AssetLoader>,
    aspect_card_datas: Res<AspectCardDatas>,
) {
    let card_list = card_list_q.get(trigger.target()).unwrap().card_list();

    let card_list_container_entity = commands
        .entity(trigger.target())
        .insert((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(40.),
                row_gap: Val::Px(10.),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            children![(
                BorderColor(Color::WHITE),
                Node {
                    border: UiRect::bottom(Val::Px(1.)),
                    ..default()
                },
                children![Text::new(card_list.to_string())],
            )],
        ))
        .id();

    let card_list_entity = commands
        .spawn((
            ChildOf(card_list_container_entity),
            ScrollingList::Grid {
                column: 6,
                spacing: 10.,
            },
            card_list.clone(),
        ))
        .id();

    let cards = card_list.get_cards(&res, &aspect_card_datas);
    card_list.spawn(commands, &cards, card_list_entity, &asset_loader);
}

fn on_deck_info_changed(
    mut commands: Commands,
    mut content_q: Query<(Entity, &CardList)>,
    res: Res<DeckBuildingResource>,
    aspect_card_datas: Res<AspectCardDatas>,
    asset_loader: Res<AssetLoader>,
) {
    if res.is_changed() {
        if res.get_deck().is_some() {
            for (entity, card_list) in content_q.iter_mut() {
                let cards = card_list.get_cards(&res, &aspect_card_datas);
                commands.entity(entity).despawn_related::<Children>();
                card_list
                    .clone()
                    .spawn(commands.reborrow(), &cards, entity, &asset_loader);
            }
        }
    }
}

fn handle_mouse_event(
    mut click_ev: EventReader<MouseControlEvent>,
    mut commands: Commands,
    card_info_q: Query<(&Card<'static>, &GlobalTransform, &CardFrom), With<MouseControl>>,
    asset_loader: Res<AssetLoader>,
) {
    for ev in click_ev.read() {
        match ev {
            MouseControlEvent::ShortClick(entity) => {
                if let Ok((card_info, _, _)) = card_info_q.get(*entity) {
                    commands.spawn(CardDetail::new(card_info.get_key(), card_info.is_vertical));
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            MouseControlEvent::StartDrag(entity) => {
                if let Ok((card_info, card_transform, card_from)) = card_info_q.get(*entity) {
                    let transform = card_transform.translation();
                    commands.spawn(DraggingCard {
                        card_id: card_info.get_key(),
                        card_image: asset_loader.get(&card_info.get_key()).clone(),
                        card_from: card_from.clone(),
                        position: (
                            Val::Px(transform.x - CARD_SIZE_SMALL.x / 2.),
                            Val::Px(transform.y - CARD_SIZE_SMALL.y / 2.),
                        ),
                    });
                } else {
                    warn!("card_info not found {:?}", entity);
                }
            }
            _ => {}
        }
    }
}

#[derive(Component, Clone)]
#[require(RelativeCursorPosition)]
pub enum CardList {
    Deck,
    Collection,
}

impl CardList {
    fn to_string(&self) -> &str {
        match self {
            CardList::Deck => "Deck",
            CardList::Collection => "Collection",
        }
    }

    fn get_cards(
        &self,
        res: &Res<DeckBuildingResource>,
        aspect_card_datas: &Res<AspectCardDatas>,
    ) -> Vec<Card<'static>> {
        let aspect_card_ids = res.get_deck().unwrap().get_card_ids();
        let aspect_cards = aspect_card_datas.get_batch_card_by_id(&aspect_card_ids);
        match self {
            CardList::Deck => {
                let identity_cards = res.get_identity().unwrap().get_deck_cards();
                [identity_cards, aspect_cards].concat()
            }
            CardList::Collection => {
                let current_aspects = DeckUtil::get_current_aspects(&aspect_cards);
                DeckUtil::get_available_cards(&aspect_card_ids, &current_aspects)
            }
        }
    }

    fn spawn(
        self,
        mut commands: Commands,
        cards: &Vec<Card<'static>>,
        parent: Entity,
        asset_loader: &Res<AssetLoader>,
    ) {
        for card in cards {
            commands.spawn((
                ChildOf(parent),
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    CardNode::small(asset_loader.get(&card.get_key()).clone()),
                    card.clone(),
                    MouseControl::default(),
                    match self {
                        CardList::Deck => CardFrom::Deck,
                        CardList::Collection => CardFrom::Collection,
                    },
                )],
            ));
        }
    }
}
