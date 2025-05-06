use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_simple_text_input::TextInputValue;

use crate::{
    flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState},
    node_ui::Popup,
    resource::AspectCardDatas,
    util::DecksStorageUtil,
};

#[derive(Component)]
#[require(Interaction)]
pub enum HeaderButton {
    Back,
    Delete,
    Save,
}

pub struct HeaderButtonPlugin;

impl Plugin for HeaderButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_header_button_click.run_if(in_state(DeckBuildingState::DeckEditor)),
        );
    }
}

fn handle_header_button_click(
    mut commands: Commands,
    header_button_q: Query<(&Interaction, &HeaderButton)>,
    text_value_q: Query<&TextInputValue>,
    mut res: ResMut<DeckBuildingResource>,
    aspect_card_datas: Res<AspectCardDatas>,
    pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<DeckBuildingState>>,
) {
    for (interaction, header_button) in header_button_q.iter() {
        if interaction == &Interaction::Pressed {
            let mut deck = res.get_deck().unwrap();
            let mut deck_storage_util = DecksStorageUtil::init(&res.get_identity().unwrap(), pkv);
            match header_button {
                HeaderButton::Delete => deck_storage_util.remove_deck(deck.get_id()),
                HeaderButton::Save => {
                    if let Ok(text_value) = text_value_q.single() {
                        deck.set_name(&text_value.0);
                    }
                    if let Err(message) = deck_storage_util.save_deck(deck, aspect_card_datas) {
                        commands.spawn(Popup::new(message));
                        return;
                    }
                }
                HeaderButton::Back => {}
            }
            res.clear_deck();
            next_state.set(DeckBuildingState::DeckMenu);
            return;
        }
    }
}
