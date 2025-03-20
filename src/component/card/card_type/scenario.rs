use bevy::prelude::Component;

use crate::component::card::Count;

#[derive(Component)]
pub enum ScenarioCardType<'a> {
    Villain {
        hit_points: Count,
        sch: u8,
        atk: u8,
        next_villain_id: Option<&'a str>,
    },
    MainSchemeA {
        next_stage_id: &'a str,
    },
    MainSchemeB {
        next_stage_id: Option<&'a str>,
    },
}
