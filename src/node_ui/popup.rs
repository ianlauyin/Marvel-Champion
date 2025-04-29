use std::time::Duration;

use bevy::prelude::*;

use crate::util::UiUtils;

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_popup_timer);
    }
}

#[derive(Component)]
pub struct Popup(Timer);

impl Popup {
    pub fn new(text: String, z_index_q: &Query<&ZIndex>) -> impl Bundle {
        (
            Self(Timer::new(Duration::from_secs(2), TimerMode::Once)),
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::End,
                margin: UiRect::bottom(Val::Px(20.)),
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            UiUtils::get_largest_z_index(&z_index_q),
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor::from(Color::srgba(0.843, 0.047, 0.047, 0.9)),
            children![Text::new(text.clone())],
        )
    }
}

fn handle_popup_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_q: Query<(&mut Popup, &mut BackgroundColor, &Children, Entity)>,
    mut text_color_q: Query<&mut TextColor>,
) {
    for (mut popup, mut color, children, entity) in popup_q.iter_mut() {
        popup.0.tick(time.delta());
        if popup.0.finished() {
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
