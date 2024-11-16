use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn spawn_button<'a, 'b>(
    child_builder: &'b mut ChildBuilder<'a>,
    text: &str,
    color: Color,
    size: (Val, Val),
) -> EntityCommands<'b> {
    let mut button = child_builder.spawn(ButtonBundle {
        style: Style {
            width: size.0,
            height: size.1,
            border: UiRect::all(Val::Px(2.)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::srgb(0.725, 0.725, 0.725)),
        border_radius: BorderRadius::all(Val::Px(10.)),
        background_color: BackgroundColor::from(color),
        ..default()
    });

    button.with_children(|button| {
        button.spawn(TextBundle::from_section(text, TextStyle::default()));
    });

    button
}
