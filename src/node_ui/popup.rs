use std::time::Duration;

use bevy::prelude::*;

use crate::util::UiUtils;

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_popup_timer)
            .add_observer(handle_popup_spawn);
    }
}

#[derive(Component)]
pub struct Popup {
    text: String,
    timer: Timer,
}

impl Popup {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
        }
    }
}

fn handle_popup_spawn(
    trigger: Trigger<OnAdd, Popup>,
    mut commands: Commands,
    popup_q: Query<&Popup>,
    z_index_q: Query<&ZIndex>,
) {
    let z_index = UiUtils::get_largest_z_index(&z_index_q);
    let popup = popup_q.get(trigger.target()).unwrap();
    commands
        .entity(trigger.target())
        .insert((
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::End,
                margin: UiRect::bottom(Val::Px(20.)),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            z_index,
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::srgba(0.843, 0.047, 0.047, 0.9)),
        ))
        .with_child(Text::new(popup.text.clone()));
}

fn handle_popup_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_q: Query<(&mut Popup, &mut BackgroundColor, &Children, Entity)>,
    mut text_color_q: Query<&mut TextColor>,
) {
    for (mut popup, mut color, children, entity) in popup_q.iter_mut() {
        popup.timer.tick(time.delta());
        if popup.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        let new_alpha = color.0.alpha() - 0.01;
        color.0.set_alpha(new_alpha);
        let mut text_color = text_color_q.get_mut(*children.get(0).unwrap()).unwrap();
        let new_text_alpha = text_color.0.alpha() - 0.01;
        text_color.set_alpha(new_text_alpha);
    }
}
