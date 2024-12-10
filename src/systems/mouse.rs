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
pub struct MouseShortClickEvent(pub Entity);

#[derive(Event)]
pub struct MouseDragEvent {
    pub entity: Entity,
    pub delta_position: Option<Vec2>,
    pub position: Vec2,
}

#[derive(Event)]
pub struct MouseDropEvent {
    pub entity: Entity,
    pub position: Vec2,
}

const TIME_BOUNDARY: f32 = 0.1;

fn listen_mouse_click(
    time: Res<Time>,
    mouse_click_writer: EventWriter<MouseShortClickEvent>,
    mouse_drag_writer: EventWriter<MouseDragEvent>,
    mouse_drop_writer: EventWriter<MouseDropEvent>,
    mut mouse_target_q: Query<(Entity, &Interaction, &mut MouseDragDropClick)>,
    mut cursor_ev: EventReader<CursorMoved>,
) {
    for (entity, interaction, mut mouse_target) in mouse_target_q.iter_mut() {
        if *interaction == Interaction::Pressed || !mouse_target.stop_watch.is_paused() {
            mouse_target.stop_watch.tick(time.delta());
            let mut delta_position = None;
            if let Some(cursor) = cursor_ev.read().next() {
                delta_position = Some(cursor.position - mouse_target.position);
                mouse_target.position = cursor.position;
            }
            if *interaction == Interaction::Pressed {
                handle_pressed(entity, mouse_drag_writer, &mut mouse_target, delta_position);
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
    delta_position: Option<Vec2>,
) {
    if mouse_target.stop_watch.elapsed_secs() >= TIME_BOUNDARY {
        mouse_drag_writer.send(MouseDragEvent {
            entity,
            position: mouse_target.position,
            delta_position,
        });
    } else if mouse_target.stop_watch.is_paused() {
        mouse_target.stop_watch.unpause();
    }
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
        mouse_drop_writer.send(MouseDropEvent {
            entity,
            position: mouse_target.position,
        });
    }
    mouse_target.stop_watch.pause();
    mouse_target.stop_watch.reset();
}
