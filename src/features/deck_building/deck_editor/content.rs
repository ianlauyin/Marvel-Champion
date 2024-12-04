use bevy::prelude::*;

use crate::systems::Deck;

use super::EditingDeck;

pub fn spawn_content(menu: &mut ChildBuilder, deck: Deck) {
    menu.spawn(Node {
        flex_grow: 1.,
        ..default()
    })
    .with_children(|content| {
        spawn_name_list(content);
        spawn_card_list(content, deck);
    });
}

fn spawn_name_list(content: &mut ChildBuilder) {}

fn spawn_card_list(content: &mut ChildBuilder, deck: Deck) {}
