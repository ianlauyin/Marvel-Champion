use std::collections::HashMap;

use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::{
    features::{
        cards::{Card, CardAspect, CardDatas},
        shared::ListItem,
    },
    systems::{LoadedAssetMap, MouseDragDropClick},
};

use super::ui::{CardListItem, ContentContainer};

#[derive(Bundle, Clone)]
pub struct DragDropCard {
    belongs: CardListItem,
    interaction: MouseDragDropClick,
    card: Card,
}

pub fn convert_card_into_button_map(
    belongs: CardListItem,
    cards: &Vec<Card>,
    loaded_asset: &Res<LoadedAssetMap>,
) -> Vec<(DragDropCard, ListItem)> {
    cards
        .iter()
        .map(|card| {
            (
                DragDropCard {
                    belongs: belongs.clone(),
                    interaction: MouseDragDropClick::default(),
                    card: card.clone(),
                },
                ListItem {
                    text: "".to_string(),
                    color: Color::NONE,
                    image: ImageNode::new(loaded_asset.0.get(&card.get_id()).unwrap().clone()),
                },
            )
        })
        .collect()
}

pub fn get_aspect_names(deck_cards: &Vec<Card>) -> Vec<(String, Color)> {
    let mut hash_map: HashMap<String, Color> = HashMap::new();
    for card in deck_cards {
        let Ok(aspect) = card.get_aspect() else {
            continue;
        };
        match aspect {
            CardAspect::Justice => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.871, 0.941, 0.086));
            }
            CardAspect::Aggression => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.741, 0.192, 0.192));
            }
            CardAspect::Protection => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.075, 0.773, 0.075));
            }
            CardAspect::Leadership => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.125, 0.769, 0.882));
            }
            CardAspect::Pool => {
                hash_map.insert(aspect.to_string(), Color::srgb(0.89, 0.149, 0.816));
            }
            _ => continue,
        }
    }
    hash_map.into_iter().collect()
}

pub fn find_card_belongs(
    content_container_q: Query<(&RelativeCursorPosition, &ContentContainer)>,
) -> Result<ContentContainer, String> {
    for (relative_cursor_position, content_container) in content_container_q.iter() {
        if relative_cursor_position.mouse_over() {
            return Ok(content_container.clone());
        }
    }
    Err("The card is not in both of the container".to_string())
}

pub fn get_selectable_cards(deck_cards: &Vec<Card>) -> Vec<Card> {
    for card in deck_cards {
        let Ok(aspect) = card.get_aspect() else {
            continue;
        };
        match aspect {
            CardAspect::IdentitySpecific(_) | CardAspect::Basic => continue,
            _ => return [CardDatas::get_basic_cards(), aspect.get_cards()].concat(),
        }
    }
    return CardDatas::get_aspect_cards();
}
