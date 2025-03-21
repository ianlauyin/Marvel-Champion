use bevy::prelude::*;

use crate::ui_component::CustomButton;

#[derive(Component)]
pub struct ContainerHeader {
    leading_button_text: String,
}

impl ContainerHeader {
    pub fn with_leading_button(text: String) -> Self {
        Self {
            leading_button_text: text,
        }
    }
}

pub struct ContainerHeaderPlugin;

impl Plugin for ContainerHeaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_container_header_added);
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
            padding: UiRect::horizontal(Val::Px(5.)),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(CustomButton::square(
                container_header.leading_button_text.clone(),
            ));
        });
}
