use bevy::prelude::*;

use crate::systems::Deck;

pub fn spawn_content(menu: &mut ChildBuilder, deck: Deck) {
    menu.spawn(Node {
        flex_grow: 1.,
        display: Display::Flex,
        overflow: Overflow::clip_y(),
        ..default()
    })
    .with_children(|content| {
        spawn_name_list(content, deck);
        spawn_card_list(content);
    });
}

fn spawn_name_list(content: &mut ChildBuilder, deck: Deck) {}

fn spawn_card_list(content: &mut ChildBuilder) {}
