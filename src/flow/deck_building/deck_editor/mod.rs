mod card_list_container;
mod dragging_card;
mod editor;
mod header_button;
mod infomation;
mod title;

use bevy::app::{App, Plugin};

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            header_button::HeaderButtonPlugin,
            editor::DeckEditorPlugin,
            title::DeckEditorTitlePlugin,
            dragging_card::DeckEditorDraggingCardPlugin,
            card_list_container::DeckEditorCardListContainerPlugin,
            infomation::DeckEditorInfomationPlugin,
        ));
    }
}
