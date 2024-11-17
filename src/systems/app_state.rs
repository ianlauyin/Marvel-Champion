use bevy::prelude::*;

#[derive(States, Default, Hash, PartialEq, Eq, Debug, Clone)]
pub enum AppState {
    #[default]
    LoadingAsset,
    MainMenu,
    Game,
    DeckBuilding,
    Collection,
    Record,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().observe(change_app_state);
    }
}

#[derive(Event, Clone)]
pub struct AppStateChangeEvent(pub AppState);

fn change_app_state(
    trigger: Trigger<AppStateChangeEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let AppStateChangeEvent(state) = trigger.event();
    next_state.set(state.clone());
}
