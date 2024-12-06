use bevy::{prelude::*, state::state::FreelyMutableState, time::Stopwatch};
pub struct MousePlugin<S: FreelyMutableState> {
    pub current_state: S,
}

impl<S: FreelyMutableState> Plugin for MousePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_mouse_click.run_if(in_state(self.current_state.clone())),
        );
    }
}

#[derive(Component)]
#[require(Interaction)]
pub struct MouseDragDropClick {
    stop_watch: Stopwatch,
    position: Vec2,
}

// Starting with pausing stopwatch
impl Default for MouseDragDropClick {
    fn default() -> Self {
        let mut stop_watch = Stopwatch::new();
        stop_watch.pause();
        Self {
            stop_watch,
            position: Vec2::ZERO,
        }
    }
}

#[derive(Event)]
pub struct MouseShortClickEvent;

#[derive(Event)]
pub struct MouseDragEvent(pub Vec2);

#[derive(Event)]
pub struct MouseDropEvent(pub Vec2);

const TIME_BOUNDARY: f32 = 0.1;

fn listen_mouse_click(
    time: Res<Time>,
    mut commands: Commands,
    mut mouse_target_q: Query<(&Interaction, &mut MouseDragDropClick)>,
    mut cursor_ev: EventReader<CursorMoved>,
) {
    for (interaction, mut mouse_target) in mouse_target_q.iter_mut() {
        mouse_target.stop_watch.tick(time.delta());
        if let Some(cursor) = cursor_ev.read().next() {
            mouse_target.position = cursor.position
        }
        if *interaction == Interaction::Pressed {
            handle_pressed(&mut commands, &mut mouse_target);
        } else if !mouse_target.stop_watch.is_paused() {
            handle_released(&mut commands, &mut mouse_target);
        }
    }
}
fn handle_pressed(commands: &mut Commands, mouse_target: &mut MouseDragDropClick) {
    if mouse_target.stop_watch.elapsed_secs() >= TIME_BOUNDARY {
        commands.trigger(MouseDragEvent(mouse_target.position))
    } else if mouse_target.stop_watch.is_paused() {
        mouse_target.stop_watch.unpause();
    }
}

fn handle_released(commands: &mut Commands, mouse_target: &mut MouseDragDropClick) {
    if mouse_target.stop_watch.elapsed_secs() < TIME_BOUNDARY {
        commands.trigger(MouseShortClickEvent);
    } else {
        commands.trigger(MouseDropEvent(mouse_target.position));
    }
    mouse_target.stop_watch.pause();
    mouse_target.stop_watch.reset();
}
