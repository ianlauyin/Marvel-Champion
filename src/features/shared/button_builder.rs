use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct ButtonUIPlugin;

impl Plugin for ButtonUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_ui);
    }
}

pub struct ButtonBuilder<'a> {
    pub text: &'a str,
    pub background_color: Color,
    pub size: (Val, Val),
    pub with_border: bool,
}

impl Default for ButtonBuilder<'_> {
    fn default() -> Self {
        Self {
            text: "",
            background_color: Color::srgb(0.235, 0.235, 0.235),
            size: (Val::Px(300.), Val::Px(100.)),
            with_border: true,
        }
    }
}

impl ButtonBuilder<'_> {
    pub fn spawn<'a>(&self, child_builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
        let mut button = child_builder.spawn(ButtonBundle {
            style: Style {
                width: self.size.0,
                height: self.size.1,
                border: UiRect::all(Val::Px(2.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(if self.with_border {
                Color::srgb(0.725, 0.725, 0.725)
            } else {
                Color::NONE
            }),
            border_radius: BorderRadius::all(Val::Px(10.)),
            background_color: BackgroundColor::from(self.background_color),
            ..default()
        });

        button.with_children(|button| {
            button.spawn(TextBundle::from_section(self.text, TextStyle::default()));
        });
        button
    }
}

fn handle_button_ui(
    mut button_q: Query<(&mut BackgroundColor, &Interaction), With<Button>>,
    mut window_q: Query<&mut Window>,
) {
    let mut window = window_q.get_single_mut().unwrap();
    let mut turn_pointer = false;

    for (mut background_color, interaction) in button_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0.set_alpha(0.7);
                turn_pointer = true
            }
            Interaction::Hovered => {
                background_color.0.set_alpha(0.5);
                turn_pointer = true;
            }
            Interaction::None => {
                background_color.0.set_alpha(1.);
            }
        }
    }

    window.cursor.icon = if turn_pointer {
        CursorIcon::Pointer
    } else {
        CursorIcon::default()
    };
}
