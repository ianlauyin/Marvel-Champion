use bevy::prelude::*;

use crate::{
    component::card::CardBasic,
    flow::state::AppState,
    ui_component::CardDetail,
    util::{MouseControl, MouseControlEvent},
};

pub struct DeckEditorContentSystemPlugin;

impl Plugin for DeckEditorContentSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_mouse_event).run_if(in_state(AppState::DeckBuilding)),
        );
    }
}

fn handle_mouse_event(
    mut click_ev: EventReader<MouseControlEvent>,
    mut commands: Commands,
    card_info_q: Query<&CardBasic<'static>, With<MouseControl>>,
) {
    for ev in click_ev.read() {
        match ev {
            MouseControlEvent::ShortClick(entity) => {
                if let Ok(card_info) = card_info_q.get(*entity) {
                    commands.spawn(CardDetail::new(card_info.get_key(), card_info.is_vertical));
                }
            }
            _ => {}
        }
    }
}
