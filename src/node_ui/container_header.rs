use bevy::prelude::*;

use crate::node_ui::CustomButton;

#[derive(Event)]
pub enum ContainerHeaderEvent {
    LeadingButtonPressed(Entity),
    TrailingButtonPressed(Entity),
}

#[derive(Component)]
pub struct ContainerHeader {
    leading_button: ContainerHeaderButton,
    trailing_button: Option<ContainerHeaderButton>,
}

impl ContainerHeader {
    pub fn with_leading_button(text: &str) -> Self {
        Self {
            leading_button: ContainerHeaderButton::Leading(text.to_string()),
            trailing_button: None,
        }
    }
}

#[derive(Component, Clone)]
pub enum ContainerHeaderButton {
    Leading(String),
    Trailing(String),
}

impl ContainerHeaderButton {
    pub fn get_text(&self) -> &str {
        match self {
            ContainerHeaderButton::Leading(text) | ContainerHeaderButton::Trailing(text) => text,
        }
    }
}

pub struct ContainerHeaderPlugin;

impl Plugin for ContainerHeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ContainerHeaderEvent>()
            .add_observer(on_container_header_added)
            .add_systems(Update, listen_container_header_pressed);
    }
}

fn on_container_header_added(
    trigger: Trigger<OnAdd, ContainerHeader>,
    mut commands: Commands,
    container_header_q: Query<&ContainerHeader>,
) {
    let container_header = container_header_q.get(trigger.entity()).unwrap();
    commands
        .entity(trigger.entity())
        .insert(Node {
            width: Val::Percent(100.),
            padding: UiRect::all(Val::Px(10.)),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            align_self: AlignSelf::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                CustomButton::square(container_header.leading_button.get_text()),
                container_header.leading_button.clone(),
            ));
        });
}

fn listen_container_header_pressed(
    mut event_writer: EventWriter<ContainerHeaderEvent>,
    mut button_q: Query<(&Interaction, &ContainerHeaderButton, &Parent)>,
) {
    for (interaction, button, parent) in button_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            let event = match button {
                ContainerHeaderButton::Leading(_) => {
                    ContainerHeaderEvent::LeadingButtonPressed(parent.get())
                }
                ContainerHeaderButton::Trailing(_) => {
                    ContainerHeaderEvent::TrailingButtonPressed(parent.get())
                }
            };
            event_writer.send(event);
        }
    }
}
