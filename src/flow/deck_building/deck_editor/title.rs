use bevy::prelude::*;
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputSubmitEvent, TextInputValue};

use crate::flow::deck_building::{resource::DeckBuildingResource, state::DeckBuildingState};

#[derive(Component)]
pub struct Title;

pub struct DeckEditorTitlePlugin;

impl Plugin for DeckEditorTitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_title_interaction,
                handle_text_input_finished,
                handle_text_input_escape,
            )
                .run_if(in_state(DeckBuildingState::DeckEditor)),
        )
        .add_observer(on_added);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, Title>,
    mut commands: Commands,
    res: Res<DeckBuildingResource>,
) {
    let deck = res.get_deck().unwrap();
    commands.entity(trigger.target()).insert((
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
        children![(
            TextInput,
            TextInputValue(deck.get_name().to_string()),
            TextInputInactive(true),
        )],
    ));
}

fn handle_title_interaction(
    title_q: Query<(&Interaction, &Children), With<Title>>,
    mut text_input_q: Query<&mut TextInputInactive>,
) {
    for (interaction, children) in title_q.iter() {
        if *interaction == Interaction::Pressed {
            for child in children.iter() {
                if let Ok(mut inactive) = text_input_q.get_mut(child) {
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
        if let Ok((mut inactive, mut value)) = text_input_q.single_mut() {
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
        if let Ok((mut inactive, mut value)) = text_input_q.single_mut() {
            value.0 = res.get_deck().unwrap().get_name().to_string();
            inactive.0 = true;
        };
    }
}
