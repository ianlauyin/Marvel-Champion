use bevy::prelude::*;

use crate::{
    features::{
        cards::{Card, CardDatas},
        shared::ScrollingList,
    },
    systems::Deck,
};

pub fn spawn_content(menu: &mut ChildBuilder, cards: Vec<Card>) {
    menu.spawn(Node {
        flex_grow: 1.,
        display: Display::Flex,
        overflow: Overflow::clip_y(),
        ..default()
    })
    .with_children(|content| {
        spawn_name_list(content, cards);
        spawn_card_list(content);
    });
}

fn spawn_name_list(content: &mut ChildBuilder, cards: Vec<Card>) {
    content
        .spawn(Node {
            width: Val::Percent(40.),
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        })
        .with_child((
            ScrollingList::default(),
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(30.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|scolling_list| for card in cards {});
}

fn spawn_card_list(content: &mut ChildBuilder) {}
