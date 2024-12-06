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
}

// Starting with pausing stopwatch
impl Default for MouseDragDropClick {
    fn default() -> Self {
        let mut stop_watch = Stopwatch::new();
        stop_watch.pause();
        Self { stop_watch }
    }
}

#[derive(Event)]
pub struct MouseShortClickEvent;

#[derive(Event)]
pub struct MouseDragEvent;

#[derive(Event)]
pub struct MouseDropEvent;

const TIME_BOUNDARY: f32 = 0.1;

fn listen_mouse_click(
    time: Res<Time>,
    mut commands: Commands,
    mut mouse_target_q: Query<(&Interaction, &mut MouseDragDropClick)>,
) {
    for (interaction, mut mouse_target) in mouse_target_q.iter_mut() {
        let stop_watch = mouse_target.stop_watch.tick(time.delta());
        if *interaction == Interaction::Pressed {
            if stop_watch.elapsed_secs() >= TIME_BOUNDARY {
                commands.trigger(MouseDragEvent)
            } else if mouse_target.stop_watch.is_paused() {
                mouse_target.stop_watch.unpause();
            }
        } else {
            if !stop_watch.is_paused() {
                if stop_watch.elapsed_secs() < TIME_BOUNDARY {
                    commands.trigger(MouseShortClickEvent);
                } else {
                    commands.trigger(MouseDropEvent);
                }
                mouse_target.stop_watch.pause();
                mouse_target.stop_watch.reset();
            }
        }
    }
}
