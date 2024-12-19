use bevy::{prelude::*, state::state::FreelyMutableState, time::Stopwatch};
pub struct MousePlugin<S: FreelyMutableState> {
    pub current_state: S,
}

impl<S: FreelyMutableState> Plugin for MousePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_event::<MouseShortClickEvent>()
            .add_event::<MouseDragEvent>()
            .add_event::<MouseDropEvent>()
            .add_systems(
                Update,
                listen_mouse_click.run_if(in_state(self.current_state.clone())),
            );
    }
}

#[derive(Component, Clone)]
#[require(Interaction)]
pub struct MouseDragDropClick {
    stop_watch: Stopwatch,
    position: Option<Vec2>,
}

// Starting with pausing stopwatch
impl Default for MouseDragDropClick {
    fn default() -> Self {
        let mut stop_watch = Stopwatch::new();
        stop_watch.pause();
        Self {
            stop_watch,
            position: None,
        }
    }
}

#[derive(Event)]
pub struct MouseShortClickEvent(pub Entity);

#[derive(Event)]
pub struct MouseDragEvent {
    pub entity: Entity,
    pub delta_position: Option<Vec2>,
}

#[derive(Event)]
pub struct MouseDropEvent;

const TIME_BOUNDARY: f32 = 0.1;

pub fn listen_mouse_click(
    time: Res<Time>,
    mouse_click_writer: EventWriter<MouseShortClickEvent>,
    mouse_drag_writer: EventWriter<MouseDragEvent>,
    mouse_drop_writer: EventWriter<MouseDropEvent>,
    mut mouse_target_q: Query<(Entity, &Interaction, &mut MouseDragDropClick)>,
    window_q: Query<&Window>,
) {
    let window = window_q.get_single().unwrap();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    for (entity, interaction, mut mouse_target) in mouse_target_q.iter_mut() {
        if *interaction == Interaction::Pressed || !mouse_target.stop_watch.is_paused() {
            mouse_target.stop_watch.tick(time.delta());
            if *interaction == Interaction::Pressed {
                handle_pressed(
                    entity,
                    mouse_drag_writer,
                    &mut mouse_target,
                    cursor_position,
                );
                return;
            }
            handle_released(
                entity,
                mouse_click_writer,
                mouse_drop_writer,
                &mut mouse_target,
            );
            return;
        }
    }
}

fn handle_pressed(
    entity: Entity,
    mut mouse_drag_writer: EventWriter<MouseDragEvent>,
    mouse_target: &mut MouseDragDropClick,
    cursor_position: Vec2,
) {
    if mouse_target.stop_watch.elapsed_secs() >= TIME_BOUNDARY {
        mouse_drag_writer.send(MouseDragEvent {
            entity,
            delta_position: get_delta_position(cursor_position, mouse_target),
        });
    } else if mouse_target.stop_watch.is_paused() {
        mouse_target.stop_watch.unpause();
    }
    mouse_target.position = Some(cursor_position);
}

fn handle_released(
    entity: Entity,
    mut mouse_click_writer: EventWriter<MouseShortClickEvent>,
    mut mouse_drop_writer: EventWriter<MouseDropEvent>,
    mouse_target: &mut MouseDragDropClick,
) {
    if mouse_target.stop_watch.elapsed_secs() < TIME_BOUNDARY {
        mouse_click_writer.send(MouseShortClickEvent(entity));
    } else {
        mouse_drop_writer.send(MouseDropEvent);
    }
    mouse_target.stop_watch.pause();
    mouse_target.stop_watch.reset();
    mouse_target.position = None;
}

fn get_delta_position(
    cursor_position: Vec2,
    mouse_target: &mut MouseDragDropClick,
) -> Option<Vec2> {
    let Some(previous_position) = mouse_target.position else {
        return None;
    };

    Some(cursor_position - previous_position)
}
