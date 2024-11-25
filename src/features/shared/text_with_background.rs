use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct TextWithBackgroundBuilder {
    pub text: String,
    pub background_color: Color,
    pub background_size: (Val, Val),
    pub padding: UiRect,
    pub style: Style,
}

impl Default for TextWithBackgroundBuilder {
    fn default() -> Self {
        Self {
            text: Default::default(),
            background_color: Color::srgb(0.3, 0.3, 0.3),
            background_size: (Val::Auto, Val::Auto),
            padding: UiRect::all(Val::Px(5.)),
            style: Default::default(),
        }
    }
}

impl TextWithBackgroundBuilder {
    pub fn spawn<'a>(&self, child_builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
        let mut style = self.style.clone();
        style.width = self.background_size.0;
        style.height = self.background_size.1;
        style.padding = self.padding;

        let mut text_with_background = child_builder.spawn(NodeBundle {
            style,
            background_color: BackgroundColor::from(self.background_color),
            border_radius: BorderRadius::all(Val::Px(10.)),
            ..default()
        });
        text_with_background.with_children(|text_background| {
            text_background.spawn(TextBundle::from_section(
                self.text.clone(),
                TextStyle::default(),
            ));
        });
        text_with_background
    }
}
