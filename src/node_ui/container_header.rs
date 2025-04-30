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

    pub fn with_both_button(leading_text: &str, trailing_text: &str) -> Self {
        Self {
            leading_button: ContainerHeaderButton::Leading(leading_text.to_string()),
            trailing_button: Some(ContainerHeaderButton::Trailing(trailing_text.to_string())),
        }
    }

    fn header_bundle(&self) -> impl Bundle {
        (
            Node {
                width: Val::Percent(100.),
                padding: UiRect::all(Val::Px(10.)),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                align_self: AlignSelf::FlexStart,
                ..default()
            },
            children![(
                CustomButton::square(self.leading_button.get_text()),
                self.leading_button.clone()
            ),],
        )
    }

    fn trailing_button_bundle(&self) -> Option<impl Bundle> {
        if let Some(trailing_button) = &self.trailing_button {
            Some(children![(
                CustomButton::square(trailing_button.get_text()),
                trailing_button.clone(),
            )])
        } else {
            None
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
            .add_observer(on_added)
            .add_systems(Update, listen_interaction);
    }
}

fn on_added(
    trigger: Trigger<OnAdd, ContainerHeader>,
    mut commands: Commands,
    container_header_q: Query<&ContainerHeader>,
) -> Result<(), BevyError> {
    let container_header = container_header_q.get(trigger.target())?;
    let header_entity = commands
        .entity(trigger.target())
        .insert(container_header.header_bundle())
        .id();
    if let Some(trailing_bundle) = container_header.trailing_button_bundle() {
        commands.spawn((trailing_bundle, ChildOf(header_entity)));
    }
    Ok(())
}

fn listen_interaction(
    mut event_writer: EventWriter<ContainerHeaderEvent>,
    mut button_q: Query<(&Interaction, &ContainerHeaderButton, &ChildOf)>,
) {
    for (interaction, button, child_of) in button_q.iter_mut() {
        if *interaction == Interaction::Pressed {
            let event = match button {
                ContainerHeaderButton::Leading(_) => {
                    ContainerHeaderEvent::LeadingButtonPressed(child_of.parent())
                }
                ContainerHeaderButton::Trailing(_) => {
                    ContainerHeaderEvent::TrailingButtonPressed(child_of.parent())
                }
            };
            event_writer.write(event);
        }
    }
}
