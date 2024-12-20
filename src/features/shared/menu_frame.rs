use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::features::shared::previous_button::PreviousButton;

use super::NextButton;

/// Reminder: Add PreviousButtonPlugin::<State>::default() in state plugin
pub struct MenuBuilder<
    T: Component,
    NS: States + FreelyMutableState,
    S: States + FreelyMutableState,
> {
    pub next_state: Option<NS>,
    pub component: T,
    pub previous_state: S,
    pub content_child: Entity,
}

impl<T: Component + Clone, S: States + FreelyMutableState, NS: States + FreelyMutableState>
    MenuBuilder<T, NS, S>
{
    pub fn spawn(&self, mut commands: Commands) -> Entity {
        let header = spawn_header(
            commands.reborrow(),
            self.previous_state.clone(),
            self.next_state.clone(),
        );
        let mut menu_frame = commands.spawn((
            self.component.clone(),
            Node {
                width: Val::Percent(90.),
                height: Val::Percent(90.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::BLACK.with_alpha(0.99)),
        ));

        menu_frame.add_children(&[header, self.content_child]).id()
    }
}

fn spawn_header<S: States + FreelyMutableState, NS: States + FreelyMutableState>(
    mut commands: Commands,
    previous_state: S,
    next_state: Option<NS>,
) -> Entity {
    commands
        .spawn(Node {
            height: Val::Percent(10.),
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        })
        .with_children(|header| {
            header.spawn(PreviousButton(previous_state));
            if let Some(next_state) = next_state {
                header.spawn(NextButton(next_state));
            };
        })
        .id()
}
