use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use bevy::winit::cursor::CursorIcon;

pub struct CustomButtonPlugin;

impl Plugin for CustomButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_ui)
            .add_observer(handle_button_added);
    }
}

#[derive(Component)]
pub struct CustomButton {
    pub text: String,
    pub text_color: Color,
    pub color: Color,
    pub size: (Val, Val),
    pub with_border: bool,
    pub image: ImageNode,
    pub border_radius: BorderRadius,
    pub node: Node,
}

impl Default for CustomButton {
    fn default() -> Self {
        Self {
            text_color: Color::WHITE,
            text: String::default(),
            color: Color::NONE,
            size: (Val::Px(300.), Val::Px(100.)),
            with_border: true,
            image: ImageNode::default(),
            border_radius: BorderRadius::all(Val::Px(10.)),
            node: Node::default(),
        }
    }
}

fn handle_button_added(
    on_add: Trigger<OnAdd, CustomButton>,
    mut commands: Commands,
    custom_button_q: Query<&CustomButton>,
) {
    let custom_button = custom_button_q.get(on_add.entity()).unwrap();
    let mut node = custom_button.node.clone();
    node.width = custom_button.size.0;
    node.height = custom_button.size.1;
    node.border = UiRect::all(Val::Px(2.));
    node.display = Display::Flex;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.justify_self = JustifySelf::Center;
    node.align_self = AlignSelf::Center;

    commands
        .entity(on_add.entity())
        .insert((
            Button,
            node.clone(),
            custom_button.image.clone(),
            BorderColor(if custom_button.with_border {
                Color::srgb(0.725, 0.725, 0.725)
            } else {
                Color::NONE
            }),
            custom_button.border_radius,
            BackgroundColor::from(custom_button.color),
        ))
        .with_children(|button_container| {
            button_container.spawn((
                Text::new(custom_button.text.clone()),
                TextColor(custom_button.text_color),
            ));
        });
}

fn handle_button_ui(
    mut commands: Commands,
    mut button_q: Query<(&mut BackgroundColor, &Interaction, &Children), With<CustomButton>>,
    mut text_color_q: Query<&mut TextColor>,
    mut window_q: Query<Entity, With<Window>>,
) {
    if button_q.is_empty() {
        return;
    }
    let window = window_q.get_single_mut().unwrap();
    let mut turn_pointer = false;

    for (background_color, interaction, children) in button_q.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                handle_button_color(background_color, children, &mut text_color_q, 0.7);
                break;
            }
            Interaction::Hovered => {
                handle_button_color(background_color, children, &mut text_color_q, 0.5);
                turn_pointer = true;
            }
            Interaction::None => {
                handle_button_color(background_color, children, &mut text_color_q, 1.);
            }
        }
    }

    commands
        .entity(window)
        .insert(CursorIcon::from(if turn_pointer {
            SystemCursorIcon::Pointer
        } else {
            SystemCursorIcon::default()
        }));
}

fn handle_button_color(
    mut background_color: Mut<BackgroundColor>,
    children: &Children,
    text_color_q: &mut Query<&mut TextColor>,
    alpha: f32,
) {
    background_color.0.set_alpha(alpha);
    for &child in children.iter() {
        let mut text_color = text_color_q.get_mut(child).unwrap();
        text_color.0.set_alpha(alpha);
    }
}
