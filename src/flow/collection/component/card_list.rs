use bevy::prelude::*;

use crate::{
    component::card::CardBasic,
    resource::AssetLoader,
    ui_component::{Card, ContainerHeader, ContainerHeaderEvent, MainContainer, ScrollingList},
};

#[derive(Component)]
pub struct CollectionCardList(Vec<CardBasic<'static>>);

impl CollectionCardList {
    pub fn new(cards: Vec<CardBasic<'static>>) -> Self {
        Self(cards)
    }
}

pub struct CardListPlugin;

impl Plugin for CardListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_header_button_click)
            .add_observer(on_card_list_added);
    }
}

fn on_card_list_added(
    trigger: Trigger<OnAdd, CollectionCardList>,
    mut commands: Commands,
    card_list_q: Query<&CollectionCardList>,
    asset_loader: Res<AssetLoader>,
) {
    let card_list = card_list_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert(MainContainer::new())
        .with_children(|container| {
            container.spawn(ContainerHeader::with_leading_button("X"));
            container
                .spawn(Node {
                    width: Val::Percent(100.),
                    flex_grow: 1.,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::scroll_y(),
                    ..default()
                })
                .with_children(|content| {
                    content
                        .spawn(ScrollingList::grid(8, 20.))
                        .with_children(|scrolling_list| {
                            for card in card_list.0.clone() {
                                scrolling_list
                                    .spawn(Node {
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    })
                                    .with_child(Card::medium(
                                        asset_loader.get(&card.get_key()).clone(),
                                    ));
                            }
                        });
                });
        });
}

fn handle_header_button_click(
    mut event_reader: EventReader<ContainerHeaderEvent>,
    mut commands: Commands,
    menu_q: Query<(Entity, &Children), With<CollectionCardList>>,
) {
    for event in event_reader.read() {
        for (entity, card_list_children) in menu_q.iter() {
            match event {
                ContainerHeaderEvent::LeadingButtonPressed(header_entity) => {
                    if card_list_children.contains(header_entity) {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
