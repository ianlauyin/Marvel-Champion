use bevy::prelude::Component;

#[derive(Component)]
pub struct StatsModifier {
    thw_or_sch: u8,
    atk: u8,
    def: u8,
}

impl StatsModifier {
    pub fn new(thw_or_sch: u8, atk: u8, def: u8) -> Self {
        Self {
            thw_or_sch,
            atk,
            def,
        }
    }
}
