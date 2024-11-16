use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct CustomButton<'a> {
    pub text: &'a str,
    pub background_color: Color,
    pub size: (Val, Val),
}

impl Default for CustomButton<'_> {
    fn default() -> Self {
        Self {
            text: "",
            background_color: Color::srgb(0.235, 0.235, 0.235),
            size: (Val::Px(300.), Val::Px(100.)),
        }
    }
}

impl CustomButton<'_> {
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
            border_color: BorderColor(Color::srgb(0.725, 0.725, 0.725)),
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
