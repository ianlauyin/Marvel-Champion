use bevy::prelude::Component;

#[derive(Component)]
pub enum ScenarioCardType<'a> {
    Villain { next_villain_id: Option<&'a str> },
    MainSchemeA { next_stage_id: &'a str },
    MainSchemeB { next_stage_id: Option<&'a str> },
}
