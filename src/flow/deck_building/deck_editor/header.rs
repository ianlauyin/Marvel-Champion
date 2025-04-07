use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputSubmitEvent, TextInputValue};

use crate::{
    flow::{
        deck_building::resource::{DeckBuildingResource, DeckBuildingState},
        state::AppState,
    },
    node_ui::CustomButton,
    util::DecksStorageUtil,
};

pub struct DeckEditorHeaderPlugin;

impl Plugin for DeckEditorHeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_header_button_click,
                handle_title_interaction,
                handle_text_input_finished,
                handle_text_input_escape,
            )
                .run_if(in_state(AppState::DeckBuilding)),
        )
        .add_observer(on_header_added);
    }
}

#[derive(Component)]
pub struct DeckEditorHeader;

#[derive(Component)]
struct TextInputContainer;
#[derive(Component)]
#[require(Interaction)]
enum HeaderButton {
    Back,
    Delete,
    Save,
}

fn on_header_added(
    trigger: Trigger<OnAdd, DeckEditorHeader>,
    mut commands: Commands,
    res: Res<DeckBuildingResource>,
) {
    let deck = res.get_deck().unwrap();
    commands
        .entity(trigger.entity())
        .insert(Node {
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            column_gap: Val::Px(5.),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            align_self: AlignSelf::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((CustomButton::square("<"), HeaderButton::Back));
            spawn_title(parent, deck.get_name());
            let mut save_button = CustomButton::medium("Save");
            save_button.set_color(Color::srgb(0.251, 0.855, 0.251));
            parent.spawn((save_button, HeaderButton::Save));
            let mut delete_button = CustomButton::medium("Delete");
            delete_button.set_color(Color::srgb(0.855, 0.251, 0.251));
            parent.spawn((delete_button, HeaderButton::Delete));
        });
}

fn spawn_title(parent: &mut ChildBuilder, name: &str) {
    parent
        .spawn((
            Node {
                flex_grow: 1.,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.)),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            BorderColor::from(Color::WHITE),
            BorderRadius::all(Val::Px(5.)),
            Interaction::default(),
            TextInputContainer,
        ))
        .with_child((
            TextInput,
            TextInputValue(name.to_string()),
            TextInputInactive(true),
        ));
}

fn handle_title_interaction(
    text_input_container_q: Query<(&Interaction, &Children), With<TextInputContainer>>,
    mut text_input_q: Query<&mut TextInputInactive>,
) {
    for (interaction, children) in text_input_container_q.iter() {
        if *interaction == Interaction::Pressed {
            for child in children.iter() {
                if let Ok(mut inactive) = text_input_q.get_mut(*child) {
                    inactive.0 = false;
                }
            }
        }
    }
}

fn handle_text_input_finished(
    mut events: EventReader<TextInputSubmitEvent>,
    mut text_input_q: Query<(&mut TextInputInactive, &mut TextInputValue)>,
    res: ResMut<DeckBuildingResource>,
) {
    for event in events.read() {
        let name = event.value.clone();
        if let Ok((mut inactive, mut value)) = text_input_q.get_single_mut() {
            res.get_deck().unwrap().set_name(&name);
            value.0 = name.clone();
            inactive.0 = true;
        };
    }
}

fn handle_text_input_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut text_input_q: Query<(&mut TextInputInactive, &mut TextInputValue)>,
    res: ResMut<DeckBuildingResource>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok((mut inactive, mut value)) = text_input_q.get_single_mut() {
            value.0 = res.get_deck().unwrap().get_name().to_string();
            inactive.0 = true;
        };
    }
}

fn handle_header_button_click(
    header_button_q: Query<(&Interaction, &HeaderButton)>,
    text_value_q: Query<&TextInputValue>,
    mut res: ResMut<DeckBuildingResource>,
    pkv: ResMut<PkvStore>,
) {
    if res.get_state() != DeckBuildingState::DeckEditor {
        return;
    }
    for (interaction, header_button) in header_button_q.iter() {
        if interaction == &Interaction::Pressed {
            let mut deck = res.get_deck().unwrap();
            let mut deck_storage_util = DecksStorageUtil::init(&res.get_identity().unwrap(), pkv);
            match header_button {
                HeaderButton::Delete => deck_storage_util.remove_deck(deck.get_id()),
                HeaderButton::Save => {
                    if let Ok(text_value) = text_value_q.get_single() {
                        deck.set_name(&text_value.0);
                    }
                    deck_storage_util.save_deck(deck);
                }
                HeaderButton::Back => {}
            }
            res.clear_deck();
            return;
        }
    }
}
