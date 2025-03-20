use bevy::prelude::Component;

#[derive(Component)]
pub struct CardCharacter {
    max_hit_points: u8,
    hit_points: u8,
    atk: u8,
    atk_con_dmg: u8,
    thw: u8,
    thw_con_dmg: u8,
    sch: u8,
}

impl CardCharacter {
    pub fn ally(hit_points: u8, thw: u8, thw_con_dmg: u8, atk: u8, atk_con_dmg: u8) -> Self {
        Self {
            max_hit_points: hit_points,
            hit_points,
            atk,
            atk_con_dmg,
            thw,
            thw_con_dmg,
            sch: 0,
        }
    }

    pub fn minion(hit_points: u8, sch: u8, atk: u8) -> Self {
        Self {
            max_hit_points: hit_points,
            hit_points,
            atk,
            sch,
            atk_con_dmg: 0,
            thw: 0,
            thw_con_dmg: 0,
        }
    }
}
