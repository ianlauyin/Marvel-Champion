use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct ButtonUIPlugin;

impl Plugin for ButtonUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_ui);
    }
}

pub struct ButtonBuilder {
    pub text: String,
    pub color: Color,
    pub image: UiImage,
    pub size: (Val, Val),
    pub with_border: bool,
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        Self {
            text: String::default(),
            color: Color::NONE,
            size: (Val::Px(300.), Val::Px(100.)),
            with_border: true,
            image: UiImage::default(),
        }
    }
}

impl ButtonBuilder {
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
            image: self.image.clone(),
            border_color: BorderColor(if self.with_border {
                Color::srgb(0.725, 0.725, 0.725)
            } else {
                Color::NONE
            }),
            border_radius: BorderRadius::all(Val::Px(10.)),
            background_color: BackgroundColor::from(self.color),
            ..default()
        });

        button.with_children(|button| {
            button.spawn(TextBundle::from_section(
                self.text.clone(),
                TextStyle::default(),
            ));
        });
        button
    }
}

fn handle_button_ui(
    mut button_q: Query<(&mut BackgroundColor, &Interaction, &Children), With<Button>>,
    mut text_q: Query<&mut Text>,
    mut window_q: Query<&mut Window>,
) {
    if button_q.is_empty() {
        return;
    }
    let mut window = window_q.get_single_mut().unwrap();
    let mut turn_pointer = false;

    for (background_color, interaction, children) in button_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                handle_button_color(background_color, children, &mut text_q, 0.7);
                break;
            }
            Interaction::Hovered => {
                handle_button_color(background_color, children, &mut text_q, 0.5);
                turn_pointer = true;
            }
            Interaction::None => {
                handle_button_color(background_color, children, &mut text_q, 1.);
            }
        }
    }

    window.cursor.icon = if turn_pointer {
        CursorIcon::Pointer
    } else {
        CursorIcon::default()
    };
}

fn handle_button_color(
    mut background_color: Mut<BackgroundColor>,
    children: &Children,
    text_q: &mut Query<&mut Text>,
    alpha: f32,
) {
    background_color.0.set_alpha(alpha);
    for &child in children.iter() {
        let mut text = text_q.get_mut(child).unwrap();
        for text_section in text.sections.iter_mut() {
            text_section.style.color.set_alpha(alpha);
        }
    }
}
