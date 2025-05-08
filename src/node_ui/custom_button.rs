use bevy::prelude::*;
use bevy::window::{PrimaryWindow, SystemCursorIcon};
use bevy::winit::cursor::CursorIcon;

#[derive(Component)]
pub struct CustomButton {
    text: String,
    text_color: Color,
    color: Color,
    with_border: bool,
    image: Option<Handle<Image>>,
    size: (Val, Val),
}

impl Default for CustomButton {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            text_color: Color::WHITE,
            color: Color::srgb(0.576, 0.576, 0.576),
            with_border: true,
            image: None,
            size: (Val::Px(300.), Val::Px(100.)),
        }
    }
}

impl CustomButton {
    pub fn large(text: &str) -> Self {
        Self {
            text: text.to_string(),
            ..default()
        }
    }

    pub fn medium(text: &str) -> Self {
        Self {
            text: text.to_string(),
            size: (Val::Px(150.), Val::Px(50.)),
            ..default()
        }
    }

    pub fn square(text: &str) -> Self {
        Self {
            text: text.to_string(),
            size: (Val::Px(50.), Val::Px(50.)),
            with_border: false,
            color: Color::srgb(0.173, 0.173, 0.173),
            ..default()
        }
    }

    pub fn set_image(&mut self, image: Handle<Image>) {
        self.image = Some(image);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn bundle(&self) -> impl Bundle {
        let image_node = if let Some(image) = self.image.clone() {
            ImageNode::new(image).with_color(Color::srgb(0.365, 0.365, 0.365))
        } else {
            ImageNode::default()
        };

        (
            Button,
            Node {
                width: self.size.0,
                height: self.size.1,
                border: UiRect::all(Val::Px(2.)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            image_node,
            BorderColor(if self.with_border {
                Color::srgb(0.725, 0.725, 0.725)
            } else {
                Color::NONE
            }),
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(self.color),
            children![(Text::new(&self.text), TextColor(self.text_color),)],
        )
    }
}

pub struct CustomButtonPlugin;

impl Plugin for CustomButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_added)
            .add_systems(Update, listen_interaction);
    }
}

fn on_added(
    on_add: Trigger<OnAdd, CustomButton>,
    mut commands: Commands,
    custom_button_q: Query<&CustomButton>,
) -> Result<(), BevyError> {
    let custom_button = custom_button_q.get(on_add.target())?;
    commands
        .entity(on_add.target())
        .insert(custom_button.bundle());
    Ok(())
}

fn listen_interaction(
    mut commands: Commands,
    mut button_q: Query<(&mut BackgroundColor, &Interaction, &Children), With<CustomButton>>,
    mut text_color_q: Query<&mut TextColor>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    if button_q.is_empty() {
        return;
    }

    let mut is_hovered = false;
    for (background_color, interaction, children) in button_q.iter_mut() {
        let alpha = match interaction {
            Interaction::Pressed => 0.7,
            Interaction::None => 1.,
            Interaction::Hovered => {
                is_hovered = true;
                0.5
            }
        };
        handle_button_color(background_color, children, &mut text_color_q, alpha);
    }

    commands.entity(window.into_inner()).insert(if is_hovered {
        CursorIcon::System(SystemCursorIcon::Pointer)
    } else {
        CursorIcon::default()
    });
}

fn handle_button_color(
    mut background_color: Mut<BackgroundColor>,
    children: &Children,
    text_color_q: &mut Query<&mut TextColor>,
    alpha: f32,
) {
    background_color.0.set_alpha(alpha);
    for child in children.iter() {
        let Ok(mut text_color) = text_color_q.get_mut(child) else {
            continue;
        };
        text_color.0.set_alpha(alpha);
    }
}
