use bevy::{prelude::*, time::Stopwatch, window::PrimaryWindow};

// TOOD: Refactor use Pointer<Drag> Event
#[derive(Component, Clone)]
#[require(Interaction)]
pub struct MouseControl {
    stop_watch: Stopwatch,
    dragging: bool,
    position: Option<Vec2>,
}

// Starting with pausing stopwatch
impl Default for MouseControl {
    fn default() -> Self {
        let mut stop_watch = Stopwatch::new();
        stop_watch.pause();
        Self {
            stop_watch,
            dragging: false,
            position: None,
        }
    }
}

#[derive(Event)]
pub enum MouseControlEvent {
    ShortClick(Entity),
    StartDrag(Entity),
    Drag(Option<Vec2>),
    Drop,
}

pub struct MouseControlUtilPlugin;

impl Plugin for MouseControlUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseControlEvent>()
            .add_systems(Update, listen_mouse_click);
    }
}

const TIME_BOUNDARY: f32 = 0.1;

pub fn listen_mouse_click(
    event_writer: EventWriter<MouseControlEvent>,
    time: Res<Time>,
    mut mouse_target_q: Query<(Entity, &Interaction, &mut MouseControl)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    for (entity, interaction, mut mouse_target) in mouse_target_q.iter_mut() {
        if *interaction == Interaction::Pressed || !mouse_target.stop_watch.is_paused() {
            mouse_target.stop_watch.tick(time.delta());
            if *interaction == Interaction::Pressed {
                handle_pressed(entity, event_writer, &mut mouse_target, cursor_position);
                return;
            }
            handle_released(entity, event_writer, &mut mouse_target);
            return;
        }
    }
}

fn handle_pressed(
    entity: Entity,
    mut event_writer: EventWriter<MouseControlEvent>,
    mouse_target: &mut MouseControl,
    cursor_position: Vec2,
) {
    if mouse_target.dragging {
        event_writer.write(MouseControlEvent::Drag(get_delta_position(
            cursor_position,
            mouse_target,
        )));
    } else if mouse_target.stop_watch.elapsed_secs() >= TIME_BOUNDARY {
        mouse_target.dragging = true;
        event_writer.write(MouseControlEvent::StartDrag(entity));
    } else if mouse_target.stop_watch.is_paused() {
        mouse_target.stop_watch.unpause();
    }
    mouse_target.position = Some(cursor_position);
}

fn handle_released(
    entity: Entity,
    mut event_writer: EventWriter<MouseControlEvent>,
    mouse_target: &mut MouseControl,
) {
    if mouse_target.stop_watch.elapsed_secs() < TIME_BOUNDARY {
        event_writer.write(MouseControlEvent::ShortClick(entity));
    } else {
        event_writer.write(MouseControlEvent::Drop);
    }
    mouse_target.stop_watch.pause();
    mouse_target.stop_watch.reset();
    mouse_target.dragging = false;
    mouse_target.position = None;
}

fn get_delta_position(cursor_position: Vec2, mouse_target: &mut MouseControl) -> Option<Vec2> {
    let Some(previous_position) = mouse_target.position else {
        return None;
    };
    Some(cursor_position - previous_position)
}
