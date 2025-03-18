use bevy::prelude::Component;

#[derive(Component)]
pub struct CardCharacter {
    hit_points: u8,
    atk: u8,
    atk_con_dmg: u8,
    thw: u8,
    thw_con_dmg: u8,
    sch: u8,
}
