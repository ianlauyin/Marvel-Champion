use bevy::{prelude::*, state::state::FreelyMutableState};

use crate::features::shared::previous_button::PreviousButtonBuilder;

use super::{spawn_card_list, spawn_list};

#[derive(Default, Clone)]
pub struct ListItem {
    pub text: String,
    pub color: Color,
    pub image: ImageNode,
}

pub enum DisplayMethod {
    ButtonList,
    CardList,
}

/// Reminder: Add handle_previous_interaction(current_state) in system
pub struct MenuBuilder<T: Component, S: States + FreelyMutableState, B: Component> {
    pub component: T,
    pub previous_state: S,
    pub list_items: Vec<(B, ListItem)>,
    pub display_method: DisplayMethod,
}

impl<T: Component + Clone, S: States + FreelyMutableState, B: Component + Clone>
    MenuBuilder<T, S, B>
{
    pub fn spawn(&self, mut commands: Commands) {
        commands
            .spawn((
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
            ))
            .with_children(|menu| {
                spawn_header(menu, self.previous_state.clone());
                match self.display_method {
                    DisplayMethod::ButtonList => spawn_list(menu, self.list_items.clone()),
                    DisplayMethod::CardList => spawn_card_list(menu, self.list_items.clone()),
                }
            });
    }
}

fn spawn_header<S: States + FreelyMutableState>(menu: &mut ChildBuilder, previous_state: S) {
    menu.spawn(Node {
        height: Val::Percent(10.),
        padding: UiRect {
            left: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            ..default()
        },
        ..default()
    })
    .with_children(|header| {
        PreviousButtonBuilder(previous_state).spawn(header);
    });
}
