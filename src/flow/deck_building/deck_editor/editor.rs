use bevy::prelude::*;

use crate::{
    flow::deck_building::state::DeckBuildingState,
    node_ui::{CustomButton, MainContainer},
    util::SystemUtil,
};

use super::{
    card_list_container::CardListContainer, header_button::HeaderButton, infomation::Infomation,
    title::Title,
};

const CURRENT_STATE: DeckBuildingState = DeckBuildingState::DeckEditor;

pub struct DeckEditorPlugin;

impl Plugin for DeckEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CURRENT_STATE), spawn_deck_editor)
            .add_systems(OnExit(CURRENT_STATE), SystemUtil::cleanup_all::<DeckEditor>);
    }
}

#[derive(Component)]
struct DeckEditor;

fn spawn_deck_editor(mut commands: Commands) {
    let main_container = commands.spawn((MainContainer::default(), DeckEditor)).id();

    let header = commands
        .spawn((
            ChildOf(main_container),
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(10.)),
                column_gap: Val::Px(5.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                align_self: AlignSelf::FlexStart,
                ..default()
            },
            children![(CustomButton::square("<"), HeaderButton::Back), Title],
        ))
        .id();

    let mut save_button = CustomButton::medium("Save");
    save_button.set_color(Color::srgb(0.251, 0.855, 0.251));
    commands.spawn((save_button, HeaderButton::Save, ChildOf(header)));

    let mut delete_button = CustomButton::medium("Delete");
    delete_button.set_color(Color::srgb(0.855, 0.251, 0.251));
    commands.spawn((delete_button, HeaderButton::Delete, ChildOf(header)));

    commands.spawn((
        ChildOf(main_container),
        Node {
            display: Display::Flex,
            column_gap: Val::Px(5.),
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            overflow: Overflow::scroll_y(),
            ..default()
        },
        children![
            CardListContainer::Deck,
            Infomation,
            CardListContainer::Collection
        ],
    ));
}
